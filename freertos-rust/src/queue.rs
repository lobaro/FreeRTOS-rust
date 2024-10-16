use mem::ManuallyDrop;
use mem::MaybeUninit;

use crate::base::*;
use crate::isr::*;
use crate::prelude::v1::*;
use crate::shim::*;
use crate::units::*;

unsafe impl<T: Sized + Send> Send for Queue<T> {}
unsafe impl<T: Sized + Send> Sync for Queue<T> {}

#[derive(Debug)]
pub struct SendError<T> {
    err: FreeRtosError,
    item: T,
}

impl<T> SendError<T> {
    pub fn error(&self) -> FreeRtosError {
        self.err
    }

    pub fn into_item(self) -> T {
        self.item
    }
}

/// A queue with a finite size.
#[derive(Debug)]
pub struct Queue<T: Sized + Send> {
    queue: FreeRtosQueueHandle,
    item_type: PhantomData<T>,
}

impl<T: Sized + Send> Queue<T> {
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
    pub fn send<D: DurationTicks>(&self, item: T, max_wait: D) -> Result<(), SendError<T>> {
        let item = ManuallyDrop::new(item);
        let ptr = &item as *const _ as FreeRtosVoidPtr;

        unsafe {
            if freertos_rs_queue_send(self.queue, ptr, max_wait.to_ticks()) != 0 {
                Err(SendError {
                    err: FreeRtosError::QueueSendTimeout,
                    item: ManuallyDrop::into_inner(item),
                })
            } else {
                Ok(())
            }
        }
    }

    /// Send an item to the end of the queue, from an interrupt.
    pub fn send_from_isr(
        &self,
        context: &mut InterruptContext,
        item: T,
    ) -> Result<(), SendError<T>> {
        let item = ManuallyDrop::new(item);
        let ptr = &item as *const _ as FreeRtosVoidPtr;

        unsafe {
            if freertos_rs_queue_send_isr(self.queue, ptr, context.get_task_field_mut()) != 0 {
                Err(SendError {
                    err: FreeRtosError::QueueFull,
                    item: ManuallyDrop::into_inner(item),
                })
            } else {
                Ok(())
            }
        }
    }

    /// Wait for an item to be available on the queue.
    pub fn receive<D: DurationTicks>(&self, max_wait: D) -> Result<T, FreeRtosError> {
        unsafe {
            // Use `MaybeUninit` to avoid calling drop on
            // uninitialized struct in case of timeout
            let mut buff = MaybeUninit::uninit();
            let r = freertos_rs_queue_receive(
                self.queue,
                &mut buff as *mut _ as FreeRtosMutVoidPtr,
                max_wait.to_ticks(),
            );
            if r == 0 {
                return Ok(buff.assume_init());
            } else {
                return Err(FreeRtosError::QueueReceiveTimeout);
            }
        }
    }

    /// Get the number of messages in the queue.
    pub fn len(&self) -> u32 {
        unsafe { freertos_rs_queue_messages_waiting(self.queue) }
    }
}

impl<T: Sized + Send> Drop for Queue<T> {
    fn drop(&mut self) {
        unsafe {
            freertos_rs_queue_delete(self.queue);
        }
    }
}
