use core::marker::PhantomData;

use crate::base::*;
use crate::isr::*;
use crate::shim::*;
use crate::units::Duration;

unsafe impl<T: Sized + Copy> Send for Queue<T> {}
unsafe impl<T: Sized + Copy> Sync for Queue<T> {}

/// A queue with a finite size. The items are owned by the queue and are
/// copied.
#[derive(Debug)]
pub struct Queue<T: Sized + Copy> {
    queue: FreeRtosQueueHandle,
    item_type: PhantomData<T>,
}

impl<T: Sized + Copy> Queue<T> {
    pub fn new(max_size: usize) -> Result<Queue<T>, FreeRtosError> {
        let item_size = core::mem::size_of::<T>();

        let handle = unsafe { freertos_rs_queue_create(max_size as u32, item_size as u32) };

        if handle.is_null() {
            return Err(FreeRtosError::OutOfMemory);
        }

        Ok(Queue {
            queue: handle,
            item_type: PhantomData,
        })
    }

    /// # Safety
    ///
    /// `handle` must be a valid FreeRTOS regular queue handle (not semaphore or mutex).
    ///
    /// The item size of the queue must match the size of `T`.
    #[inline]
    pub unsafe fn from_raw_handle(handle: FreeRtosQueueHandle) -> Self {
        Self {
            queue: handle,
            item_type: PhantomData,
        }
    }
    #[inline]
    pub fn raw_handle(&self) -> FreeRtosQueueHandle {
        self.queue
    }

    /// Send an item to the end of the queue. Wait for the queue to have empty space for it.
    pub fn send(&self, item: T, max_wait: Duration) -> Result<(), FreeRtosError> {
        match unsafe {
            freertos_rs_queue_send(
                self.queue,
                &item as *const _ as FreeRtosVoidPtr,
                max_wait.ticks(),
            )
        } {
            0 => Ok(()),
            _ => Err(FreeRtosError::QueueSendTimeout),
        }
    }

    /// Send an item to the end of the queue, from an interrupt.
    pub fn send_from_isr(
        &self,
        context: &mut InterruptContext,
        item: T,
    ) -> Result<(), FreeRtosError> {
        match unsafe {
            freertos_rs_queue_send_isr(
                self.queue,
                &item as *const _ as FreeRtosVoidPtr,
                context.get_task_field_mut(),
            )
        } {
            0 => Ok(()),
            _ => Err(FreeRtosError::QueueFull),
        }
    }

    /// Wait for an item to be available on the queue.
    pub fn receive(&self, max_wait: Duration) -> Result<T, FreeRtosError> {
        let mut buff = unsafe { core::mem::zeroed::<T>() };

        match unsafe {
            freertos_rs_queue_receive(
                self.queue,
                &mut buff as *mut _ as FreeRtosMutVoidPtr,
                max_wait.ticks(),
            )
        } {
            0 => Ok(buff),
            _ => Err(FreeRtosError::QueueReceiveTimeout),
        }
    }

    /// Get the number of messages in the queue.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> u32 {
        unsafe { freertos_rs_queue_messages_waiting(self.queue) }
    }
}

impl<T: Sized + Copy> Drop for Queue<T> {
    fn drop(&mut self) {
        unsafe {
            freertos_rs_queue_delete(self.queue);
        }
    }
}
