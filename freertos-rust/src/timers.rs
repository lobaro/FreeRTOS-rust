use alloc::boxed::Box;
use alloc::string::String;

use crate::base::*;
use crate::shim::*;
use crate::units::Duration;
use crate::InterruptContext;

unsafe impl Send for Timer {}
unsafe impl Sync for Timer {}

/// A FreeRTOS software timer.
///
/// Note that all operations on a timer are processed by a FreeRTOS internal task
/// that receives messages in a queue. Every operation has an associated waiting time
/// for that queue to get unblocked.
pub struct Timer {
    handle: FreeRtosTimerHandle,
}

/// Helper builder for a new software timer.
pub struct TimerBuilder {
    name: String,
    period: Duration,
    auto_reload: bool,
}

impl TimerBuilder {
    /// Set the name of the timer.
    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = name.into();
        self
    }

    /// Set the period of the timer.
    pub fn set_period(&mut self, period: Duration) -> &mut Self {
        self.period = period;
        self
    }

    /// Should the timer be automatically reloaded?
    pub fn set_auto_reload(&mut self, auto_reload: bool) -> &mut Self {
        self.auto_reload = auto_reload;
        self
    }

    /// Try to create the new timer.
    ///
    /// Note that the newly created timer must be started.
    pub fn create<F>(&self, callback: F) -> Result<Timer, FreeRtosError>
    where
        F: Fn(Timer),
        F: Send + 'static,
    {
        Timer::spawn(
            self.name.as_str(),
            self.period.ticks(),
            self.auto_reload,
            callback,
        )
    }
}

impl Timer {
    /// Create a new timer builder.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(period: Duration) -> TimerBuilder {
        TimerBuilder {
            name: "timer".into(),
            period,
            auto_reload: true,
        }
    }

    /// Create a timer from a raw handle.
    ///
    /// # Safety
    ///
    /// `handle` must be a valid FreeRTOS timer handle.
    #[inline]
    pub unsafe fn from_raw_handle(handle: FreeRtosTimerHandle) -> Self {
        Self { handle }
    }

    #[inline]
    pub fn raw_handle(&self) -> FreeRtosTimerHandle {
        self.handle
    }

    unsafe fn spawn_inner<'a>(
        name: &str,
        period_ticks: FreeRtosTickType,
        auto_reload: bool,
        callback: Box<dyn Fn(Timer) + Send + 'a>,
    ) -> Result<Timer, FreeRtosError> {
        let f = Box::new(callback);
        let param_ptr = &*f as *const _ as *mut _;

        let (success, timer_handle) = {
            let name = name.as_bytes();
            let name_len = name.len();

            let ret = freertos_rs_timer_create(
                name.as_ptr(),
                name_len as u8,
                period_ticks,
                if auto_reload { 1 } else { 0 },
                param_ptr,
                timer_callback,
            );

            ((ret as usize) != 0, ret)
        };

        if success {
            core::mem::forget(f);
        } else {
            return Err(FreeRtosError::OutOfMemory);
        }

        extern "C" fn timer_callback(handle: FreeRtosTimerHandle) {
            let timer = Timer { handle };
            if let Ok(callback_ptr) = timer.get_id() {
                unsafe {
                    let b = Box::from_raw(callback_ptr as *mut Box<dyn Fn(Timer)>);
                    b(timer);
                    // # TODO
                    // Investigate what is happening here and document.
                    Box::into_raw(b);
                }
            }
        }

        Ok(Timer {
            handle: timer_handle as *const _,
        })
    }

    fn spawn<F>(
        name: &str,
        period_tick: FreeRtosTickType,
        auto_reload: bool,
        callback: F,
    ) -> Result<Timer, FreeRtosError>
    where
        F: Fn(Timer),
        F: Send + 'static,
    {
        unsafe { Timer::spawn_inner(name, period_tick, auto_reload, Box::new(callback)) }
    }

    /// Start the timer.
    pub fn start(&self, block_time: Duration) -> Result<(), FreeRtosError> {
        match unsafe { freertos_rs_timer_start(self.handle, block_time.ticks()) } {
            0 => Ok(()),
            _ => Err(FreeRtosError::Timeout),
        }
    }

    /// Start the timer from an interrupt.
    pub fn start_from_isr(&self, context: &mut InterruptContext) -> Result<(), FreeRtosError> {
        match unsafe { freertos_rs_timer_start_from_isr(self.handle, context.get_task_field_mut()) }
        {
            0 => Ok(()),
            _ => Err(FreeRtosError::QueueSendTimeout),
        }
    }

    /// Stop the timer.
    pub fn stop(&self, block_time: Duration) -> Result<(), FreeRtosError> {
        match unsafe { freertos_rs_timer_stop(self.handle, block_time.ticks()) } {
            0 => Ok(()),
            _ => Err(FreeRtosError::Timeout),
        }
    }

    /// Change the period of the timer.
    pub fn change_period(
        &self,
        block_time: Duration,
        new_period: Duration,
    ) -> Result<(), FreeRtosError> {
        match unsafe {
            freertos_rs_timer_change_period(self.handle, block_time.ticks(), new_period.ticks())
        } {
            0 => Ok(()),
            _ => Err(FreeRtosError::Timeout),
        }
    }

    /// Detach this timer from Rust's memory management. The timer will still be active and
    /// will consume the memory.
    ///
    /// Can be used for timers that will never be changed and don't need to stay in scope.
    ///
    /// This method is safe because resource leak is safe in Rust.
    pub fn detach(self) {
        core::mem::forget(self);
    }

    fn get_id(&self) -> Result<FreeRtosVoidPtr, FreeRtosError> {
        unsafe { Ok(freertos_rs_timer_get_id(self.handle)) }
    }
}

impl Drop for Timer {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        if let Ok(callback_ptr) = self.get_id() {
            // free the memory
            unsafe {
                Box::from_raw(callback_ptr as *mut Box<dyn Fn(Timer)>);
            }
        }

        // todo: configurable timeout?
        unsafe {
            freertos_rs_timer_delete(self.handle, Duration::from_ms(1000).ticks());
        }
    }
}
