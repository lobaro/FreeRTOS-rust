use core::ffi::c_void;

/// Basic error type for the library.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FreeRtosError {
    OutOfMemory,
    QueueSendTimeout,
    QueueReceiveTimeout,
    MutexTimeout,
    Timeout,
    QueueFull,
    StringConversionError,
    TaskNotFound,
    InvalidQueueSize,
    ProcessorHasShutDown,
}

pub type FreeRtosVoidPtr = *const c_void;
pub type FreeRtosMutVoidPtr = *mut c_void;
pub type FreeRtosCharPtr = *const u8;
pub type FreeRtosChar = u8;

pub type FreeRtosBaseType = i32;
pub type FreeRtosUBaseType = u32;
pub type FreeRtosTickType = u32;
pub type FreeRtosBaseTypeMutPtr = *mut FreeRtosBaseType;

pub type FreeRtosTaskHandle = *const c_void;
pub type FreeRtosQueueHandle = *const c_void;
pub type FreeRtosSemaphoreHandle = *const c_void;
pub type FreeRtosTaskFunction = *const c_void;
pub type FreeRtosTimerHandle = *const c_void;
pub type FreeRtosTimerCallback = *const c_void;
#[allow(dead_code)]
pub type FreeRtosStackType = *const c_void;

pub type FreeRtosUnsignedLong = u32;
pub type FreeRtosUnsignedShort = u16;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FreeRtosTaskStatusFfi {
    pub handle: FreeRtosTaskHandle,
    pub task_name: FreeRtosCharPtr,
    pub task_number: FreeRtosUBaseType,
    pub task_state: FreeRtosTaskState,
    pub current_priority: FreeRtosUBaseType,
    pub base_priority: FreeRtosUBaseType,
    pub run_time_counter: FreeRtosUnsignedLong,
    pub stack_base: FreeRtosCharPtr,
    pub stack_high_water_mark: FreeRtosUnsignedShort,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum FreeRtosTaskState {
    /// A task is querying the state of itself, so must be running.
    Running = 0,
    /// The task being queried is in a read or pending ready list.
    Ready = 1,
    /// The task being queried is in the Blocked state.
    Blocked = 2,
    /// The task being queried is in the Suspended state, or is in the Blocked state with an infinite time out.
    Suspended = 3,
    /// The task being queried has been deleted, but its TCB has not yet been freed.
    Deleted = 4,
}
