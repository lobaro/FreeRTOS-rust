use crate::base::*;
use crate::isr::*;
use crate::prelude::v1::*;
use crate::shim::*;
use crate::units::*;

unsafe impl<T: Sized> Send for Queue<T> {}
unsafe impl<T: Sized> Sync for Queue<T> {}

/// A queue with a finite size. The items are owned by the queue and are
/// copied.
#[derive(Debug)]
pub struct Queue<T: Sized> {
    queue: FreeRtosQueueHandle,
    item_type: PhantomData<T>,
}

impl<T: Sized> Queue<T> {
    pub fn new(max_size: usize) -> Result<Queue<T>, FreeRtosError> {
        let item_size = mem::size_of::<T>();

        let handle = unsafe { freertos_rs_queue_create(max_size as u32, item_size as u32) };

        if handle == 0 as *const _ {
            return Err(FreeRtosError::OutOfMemory);
        }

        Ok(Queue {
            queue: handle,
            item_type: PhantomData,
        })
    }

    /// Send an item to the end of the queue. Wait for the queue to have empty space for it.
    pub fn send<D: DurationTicks>(&self, item: T, max_wait: D) -> Result<(), FreeRtosError> {
        unsafe {
            let ret = if freertos_rs_queue_send(
                self.queue,
                &item as *const _ as FreeRtosVoidPtr,
                max_wait.to_ticks(),
            ) != 0
            {
                Err(FreeRtosError::QueueSendTimeout)
            } else {
                Ok(())
            };

            mem::forget(item);

            ret
        }
    }

    /// Send an item to the end of the queue, from an interrupt.
    pub fn send_from_isr(
        &self,
        context: &mut InterruptContext,
        item: T,
    ) -> Result<(), FreeRtosError> {
        let ret = unsafe {
            if freertos_rs_queue_send_isr(
                self.queue,
                &item as *const _ as FreeRtosVoidPtr,
                context.get_task_field_mut(),
            ) != 0
            {
                Err(FreeRtosError::QueueFull)
            } else {
                Ok(())
            }
        };

        mem::forget(item);

        ret
    }

    /// Wait for an item to be available on the queue.
    pub fn receive<D: DurationTicks>(&self, max_wait: D) -> Result<T, FreeRtosError> {
        unsafe {
            let mut buff = mem::zeroed::<T>();
            let r = freertos_rs_queue_receive(
                self.queue,
                &mut buff as *mut _ as FreeRtosMutVoidPtr,
                max_wait.to_ticks(),
            );

            if r == 0 {
                Ok(buff)
            } else {
                Err(FreeRtosError::QueueReceiveTimeout)
            }
        }
    }
}

impl<T: Sized> Drop for Queue<T> {
    fn drop(&mut self) {
        // Drop contents
        while self.receive(Duration::ticks(0)).is_ok() {}
        unsafe {
            freertos_rs_queue_delete(self.queue);
        }
    }
}
