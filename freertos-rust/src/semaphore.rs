use crate::base::*;
use crate::isr::*;
use crate::shim::*;
use crate::units::*;

/// A counting or binary semaphore
pub struct Semaphore {
    semaphore: FreeRtosSemaphoreHandle,
}

unsafe impl Send for Semaphore {}
unsafe impl Sync for Semaphore {}

impl Semaphore {
    /// Create a new binary semaphore
    pub fn new_binary() -> Result<Semaphore, FreeRtosError> {
        unsafe {
            let s = freertos_rs_create_binary_semaphore();
            if s == 0 as *const _ {
                return Err(FreeRtosError::OutOfMemory);
            }
            Ok(Semaphore { semaphore: s })
        }
    }

    /// Create a new counting semaphore
    pub fn new_counting(max: u32, initial: u32) -> Result<Semaphore, FreeRtosError> {
        unsafe {
            let s = freertos_rs_create_counting_semaphore(max, initial);
            if s == 0 as *const _ {
                return Err(FreeRtosError::OutOfMemory);
            }
            Ok(Semaphore { semaphore: s })
        }
    }

    /// # Safety
    ///
    /// `handle` must be a valid FreeRTOS semaphore handle.
    ///
    /// Only binary or counting semaphore is expected here.
    /// To create mutex from raw handle use [`crate::mutex::MutexInnerImpl::from_raw_handle`].
    #[inline]
    pub unsafe fn from_raw_handle(handle: FreeRtosSemaphoreHandle) -> Self {
        Self { semaphore: handle }
    }
    #[inline]
    pub fn raw_handle(&self) -> FreeRtosSemaphoreHandle {
        self.semaphore
    }

    /// Lock this semaphore in a RAII fashion
    pub fn lock<D: DurationTicks>(&self, max_wait: D) -> Result<SemaphoreGuard, FreeRtosError> {
        self.take(max_wait).map(|()| SemaphoreGuard { owner: self })
    }

    /// Returns `true` on success, `false` when semaphore count already reached its limit
    pub fn give(&self) -> bool {
        unsafe { freertos_rs_give_semaphore(self.semaphore) == 0 }
    }

    pub fn take<D: DurationTicks>(&self, max_wait: D) -> Result<(), FreeRtosError> {
        unsafe {
            let res = freertos_rs_take_semaphore(self.semaphore, max_wait.to_ticks());

            if res != 0 {
                return Err(FreeRtosError::Timeout);
            }

            Ok(())
        }
    }

    /// Returns `true` on success, `false` when semaphore count already reached its limit
    pub fn give_from_isr(&self, context: &mut InterruptContext) -> bool {
        unsafe { freertos_rs_give_semaphore_isr(self.semaphore, context.get_task_field_mut()) == 0 }
    }

    /// Returns `true` on success, `false` if the semaphore was not successfully taken because it was not available
    pub fn take_from_isr(&self, context: &mut InterruptContext) -> bool {
        unsafe { freertos_rs_take_semaphore_isr(self.semaphore, context.get_task_field_mut()) == 0 }
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        unsafe {
            freertos_rs_delete_semaphore(self.semaphore);
        }
    }
}

/// Holds the lock to the semaphore until we are dropped
pub struct SemaphoreGuard<'a> {
    owner: &'a Semaphore,
}

impl<'a> Drop for SemaphoreGuard<'a> {
    fn drop(&mut self) {
        self.owner.give();
    }
}
