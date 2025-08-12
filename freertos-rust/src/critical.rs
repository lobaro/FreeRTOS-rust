use crate::base::*;
use crate::prelude::v1::*;
use crate::shim::*;

pub struct CriticalRegion;
impl CriticalRegion {
    pub fn enter() -> Self {
        unsafe {
            freertos_rs_enter_critical();
        }

        CriticalRegion
    }
}

impl Drop for CriticalRegion {
    fn drop(&mut self) {
        unsafe {
            freertos_rs_exit_critical();
        }
    }
}

unsafe impl<T: Send> Send for ExclusiveData<T> {}
unsafe impl<T: Send> Sync for ExclusiveData<T> {}

/// Data protected with a critical region. Lightweight version of a mutex,
/// intended for simple data structures.
pub struct ExclusiveData<T: ?Sized> {
    data: UnsafeCell<T>,
}

impl<T> ExclusiveData<T> {
    pub fn new(data: T) -> Self {
        ExclusiveData {
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> Result<ExclusiveDataGuard<'_, T>, FreeRtosError> {
        Ok(ExclusiveDataGuard {
            __data: &self.data,
            __lock: CriticalRegion::enter(),
        })
    }

    pub fn lock_from_isr(
        &self,
        _context: &mut crate::isr::InterruptContext,
    ) -> Result<ExclusiveDataGuardIsr<'_, T>, FreeRtosError> {
        Ok(ExclusiveDataGuardIsr { __data: &self.data })
    }
}

/// Holds the mutex until we are dropped
pub struct ExclusiveDataGuard<'a, T: ?Sized + 'a> {
    __data: &'a UnsafeCell<T>,
    __lock: CriticalRegion,
}

impl<'mutex, T: ?Sized> Deref for ExclusiveDataGuard<'mutex, T> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        unsafe { &*self.__data.get() }
    }
}

impl<'mutex, T: ?Sized> DerefMut for ExclusiveDataGuard<'mutex, T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        unsafe { &mut *self.__data.get() }
    }
}

pub struct ExclusiveDataGuardIsr<'a, T: ?Sized + 'a> {
    __data: &'a UnsafeCell<T>,
}

impl<'mutex, T: ?Sized> Deref for ExclusiveDataGuardIsr<'mutex, T> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        unsafe { &*self.__data.get() }
    }
}

impl<'mutex, T: ?Sized> DerefMut for ExclusiveDataGuardIsr<'mutex, T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        unsafe { &mut *self.__data.get() }
    }
}

unsafe impl<T: Send> Send for SuspendScheduler<T> {}
unsafe impl<T: Send> Sync for SuspendScheduler<T> {}

/// Data protected with a critical region, implemented by suspending the
/// FreeRTOS scheduler.
pub struct SuspendScheduler<T: ?Sized> {
    data: UnsafeCell<T>,
}

impl<T> SuspendScheduler<T> {
    pub const fn new(data: T) -> Self {
        SuspendScheduler {
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> SuspendSchedulerGuard<'_,T> {
        unsafe {
            freertos_rs_vTaskSuspendAll();
        }
        SuspendSchedulerGuard { data: &self.data }
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.data.get_mut()
    }

    pub fn into_inner(self) -> T {
        self.data.into_inner()
    }
}

pub struct SuspendSchedulerGuard<'a, T: ?Sized + 'a> {
    data: &'a UnsafeCell<T>,
}

impl<'mutex, T: ?Sized> Deref for SuspendSchedulerGuard<'mutex, T> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        unsafe { &*self.data.get() }
    }
}

impl<'mutex, T: ?Sized> DerefMut for SuspendSchedulerGuard<'mutex, T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        unsafe { &mut *self.data.get() }
    }
}

impl<'mutex, T: ?Sized> Drop for SuspendSchedulerGuard<'mutex, T> {
    fn drop(&mut self) {
        unsafe {
            freertos_rs_xTaskResumeAll();
        }
    }
}
