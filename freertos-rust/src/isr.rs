use crate::base::*;
use crate::shim::*;

/// Keep track of whether we need to yield the execution to a different
/// task at the end of the interrupt.
///
/// Should be dropped as the last thing inside a interrupt.
pub struct InterruptContext {
    x_higher_priority_task_woken: FreeRtosBaseType,
}

impl InterruptContext {
    /// Instantiate a new context.
    pub fn new() -> InterruptContext {
        InterruptContext {
            x_higher_priority_task_woken: 0,
        }
    }

    pub fn get_task_field_mut(&mut self) -> FreeRtosBaseTypeMutPtr {
        &mut self.x_higher_priority_task_woken as *mut _
    }
    pub fn higher_priority_task_woken(&self) -> FreeRtosBaseType {
        self.x_higher_priority_task_woken
    }
}

impl Drop for InterruptContext {
    fn drop(&mut self) {
        unsafe {
            freertos_rs_isr_yield(self.x_higher_priority_task_woken);
        }
    }
}
