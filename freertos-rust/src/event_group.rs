use crate::base::*;
use crate::shim::*;
use crate::units::*;

/// An event group
pub struct EventGroup {
    event_group: FreeRtosEventGroupHandle,
}

unsafe impl Send for EventGroup {}
unsafe impl Sync for EventGroup {}

impl EventGroup {
    /// Create a new event group
    pub fn new() -> Result<EventGroup, FreeRtosError> {
        unsafe {
            let s = freertos_rs_event_group_create();
            if s == 0 as *const _ {
                return Err(FreeRtosError::OutOfMemory);
            }
            Ok(EventGroup { event_group: s })
        }
    }

    /// # Safety
    ///
    /// `handle` must be a valid FreeRTOS event group handle.
    #[inline]
    pub unsafe fn from_raw_handle(handle: FreeRtosEventGroupHandle) -> Self {
        Self { event_group: handle }
    }
    #[inline]
    pub fn raw_handle(&self) -> FreeRtosEventGroupHandle {
        self.event_group
    }

    pub fn set_bits(&self, bits_to_set: FreeRtosEventBitsType) -> FreeRtosEventBitsType {
        unsafe { freertos_rs_event_group_set_bits(self.event_group, bits_to_set) }
    }

    pub fn get_bits(&self) -> FreeRtosEventBitsType {
        unsafe { freertos_rs_event_group_get_bits(self.event_group) }
    }

    pub fn clear_bits(&self, bits_to_clear: FreeRtosEventBitsType) -> FreeRtosEventBitsType {
        unsafe { freertos_rs_event_group_clear_bits(self.event_group, bits_to_clear) }
    }

    pub fn wait_bits<D: DurationTicks>(&self, bits_to_wait_for: FreeRtosEventBitsType, clear_on_exit: FreeRtosBaseType, wait_for_all_bits: FreeRtosBaseType, duration: D) -> FreeRtosEventBitsType {
        unsafe { freertos_rs_event_group_wait_bits(self.event_group, bits_to_wait_for, clear_on_exit, wait_for_all_bits, duration.to_ticks()) }
    }

    pub fn sync<D: DurationTicks>(&self, bits_to_set: FreeRtosEventBitsType, bits_to_wait_for: FreeRtosEventBitsType, duration: D) -> FreeRtosEventBitsType {
        unsafe { freertos_rs_event_group_sync(self.event_group, bits_to_set, bits_to_wait_for, duration.to_ticks()) }
    }
}

impl Drop for EventGroup {
    fn drop(&mut self) {
        unsafe {
            freertos_rs_event_group_delete(self.event_group);
        }
    }
}
