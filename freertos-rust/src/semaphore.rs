use crate::base::*;
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

    #[inline]
    pub unsafe fn from_raw_handle(handle: FreeRtosSemaphoreHandle) -> Self {
        Self { semaphore: handle }
    }

    /// Lock this semaphore in a RAII fashion
    pub fn lock<D: DurationTicks>(&self, max_wait: D) -> Result<SemaphoreGuard, FreeRtosError> {
        unsafe {
            let res = freertos_rs_take_mutex(self.semaphore, max_wait.to_ticks());

            if res != 0 {
                return Err(FreeRtosError::Timeout);
            }

            Ok(SemaphoreGuard { owner: self })
        }
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
        unsafe {
            freertos_rs_give_mutex(self.owner.semaphore);
        }
    }
}
