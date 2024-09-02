//! Contains manually generated freeRTOS bindings.
#![allow(non_snake_case)]

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

extern "C" {
    pub fn freertos_rs_invoke_configASSERT();
    pub fn freertos_rs_vTaskStartScheduler() -> !;
    pub fn freertos_rt_xTaskGetSchedulerState() -> FreeRtosBaseType;
    pub fn freertos_rs_pvPortMalloc(xWantedSize: FreeRtosUBaseType) -> FreeRtosVoidPtr;
    pub fn freertos_rs_vPortFree(pv: FreeRtosVoidPtr);

    pub fn freertos_rs_sizeof(_type: u8) -> u8;

    pub fn freertos_rs_vTaskDelayUntil(
        pxPreviousWakeTime: *mut FreeRtosTickType,
        xTimeIncrement: FreeRtosTickType,
    );
    pub fn freertos_rs_vTaskDelay(xTicksToDelay: FreeRtosTickType);
    #[cfg(feature = "cpu_clock")]
    pub fn freertos_rs_get_configCPU_CLOCK_HZ() -> FreeRtosUnsignedLong;
    pub fn freertos_rs_get_portTICK_PERIOD_MS() -> FreeRtosTickType;

    pub fn freertos_rs_get_number_of_tasks() -> FreeRtosUBaseType;

    pub fn freertos_rs_xTaskGetTickCount() -> FreeRtosTickType;

    pub fn freertos_rs_create_recursive_mutex() -> FreeRtosSemaphoreHandle;
    pub fn freertos_rs_create_mutex() -> FreeRtosSemaphoreHandle;

    pub fn freertos_rs_take_recursive_semaphore(
        semaphore: FreeRtosSemaphoreHandle,
        max: FreeRtosTickType,
    ) -> FreeRtosBaseType;
    pub fn freertos_rs_take_semaphore(
        semaphore: FreeRtosSemaphoreHandle,
        max: FreeRtosTickType,
    ) -> FreeRtosBaseType;
    pub fn freertos_rs_give_semaphore(semaphore: FreeRtosSemaphoreHandle) -> FreeRtosBaseType;
    pub fn freertos_rs_give_recursive_semaphore(
        semaphore: FreeRtosSemaphoreHandle,
    ) -> FreeRtosBaseType;

    pub fn freertos_rs_take_semaphore_isr(
        semaphore: FreeRtosSemaphoreHandle,
        xHigherPriorityTaskWoken: FreeRtosBaseTypeMutPtr,
    ) -> FreeRtosBaseType;
    pub fn freertos_rs_give_semaphore_isr(
        semaphore: FreeRtosSemaphoreHandle,
        xHigherPriorityTaskWoken: FreeRtosBaseTypeMutPtr,
    ) -> FreeRtosBaseType;

    pub fn freertos_rs_delete_semaphore(semaphore: FreeRtosSemaphoreHandle);

    pub fn freertos_rs_create_binary_semaphore() -> FreeRtosSemaphoreHandle;
    pub fn freertos_rs_create_counting_semaphore(
        max: FreeRtosUBaseType,
        initial: FreeRtosUBaseType,
    ) -> FreeRtosSemaphoreHandle;

    pub fn freertos_rs_queue_create(
        length: FreeRtosUBaseType,
        item_size: FreeRtosUBaseType,
    ) -> FreeRtosQueueHandle;
    pub fn freertos_rs_queue_delete(queue: FreeRtosQueueHandle);
    pub fn freertos_rs_queue_send(
        queue: FreeRtosQueueHandle,
        item: FreeRtosVoidPtr,
        max_wait: FreeRtosTickType,
    ) -> FreeRtosUBaseType;
    pub fn freertos_rs_queue_receive(
        queue: FreeRtosQueueHandle,
        item: FreeRtosMutVoidPtr,
        max_wait: FreeRtosTickType,
    ) -> FreeRtosUBaseType;
    pub fn freertos_rs_queue_messages_waiting(queue: FreeRtosQueueHandle) -> FreeRtosUBaseType;

    pub fn freertos_rs_queue_send_isr(
        queue: FreeRtosQueueHandle,
        item: FreeRtosVoidPtr,
        xHigherPriorityTaskWoken: FreeRtosBaseTypeMutPtr,
    ) -> FreeRtosUBaseType;
    pub fn freertos_rs_isr_yield();

    pub fn freertos_rs_task_notify_take(clear_count: u8, wait: FreeRtosTickType) -> u32;
    pub fn freertos_rs_task_notify_wait(
        ulBitsToClearOnEntry: u32,
        ulBitsToClearOnExit: u32,
        pulNotificationValue: *mut u32,
        xTicksToWait: FreeRtosTickType,
    ) -> FreeRtosBaseType;

    pub fn freertos_rs_task_notify(
        task: FreeRtosTaskHandle,
        value: u32,
        action: u8,
    ) -> FreeRtosBaseType;
    pub fn freertos_rs_task_notify_isr(
        task: FreeRtosTaskHandle,
        value: u32,
        action: u8,
        xHigherPriorityTaskWoken: FreeRtosBaseTypeMutPtr,
    ) -> FreeRtosBaseType;

    pub fn freertos_rs_spawn_task(
        f: extern "C" fn(FreeRtosMutVoidPtr) -> FreeRtosMutVoidPtr,
        value: FreeRtosMutVoidPtr,
        name: FreeRtosCharPtr,
        name_len: u8,
        stack_size: u16,
        priority: FreeRtosUBaseType,
        task_handle: *mut FreeRtosTaskHandle,
    ) -> FreeRtosUBaseType;
    pub fn freertos_rs_delete_task(task: FreeRtosTaskHandle);
    pub fn freertos_rs_suspend_task(task: FreeRtosTaskHandle);
    pub fn freertos_rs_vTaskSuspendAll();
    pub fn freertos_rs_xTaskResumeAll() -> FreeRtosBaseType;

    pub fn freertos_rs_uxTaskGetTaskNumber(task_handle: FreeRtosTaskHandle) -> FreeRtosBaseType;
    pub fn freertos_rs_vTaskSetTaskNumber(
        task_handle: FreeRtosTaskHandle,
        value: FreeRtosUBaseType,
    );

    pub fn freertos_rs_task_get_name(task: FreeRtosTaskHandle) -> FreeRtosCharPtr;
    pub fn freertos_rs_get_stack_high_water_mark(task: FreeRtosTaskHandle) -> FreeRtosBaseType;

    pub fn freertos_rs_get_current_task() -> FreeRtosTaskHandle;
    pub fn freertos_rs_get_system_state(
        tasks: *mut FreeRtosTaskStatusFfi,
        tasks_len: FreeRtosUBaseType,
        total_run_time: *mut u32,
    ) -> FreeRtosUBaseType;

    pub fn freertos_rs_max_wait() -> FreeRtosTickType;

    pub fn freertos_rs_timer_create(
        name: FreeRtosCharPtr,
        name_len: u8,
        period: FreeRtosTickType,
        auto_reload: u8,
        timer_id: FreeRtosVoidPtr,
        callback: extern "C" fn(FreeRtosTimerHandle) -> (),
    ) -> FreeRtosTimerHandle;
    pub fn freertos_rs_timer_start(
        timer: FreeRtosTimerHandle,
        block_time: FreeRtosTickType,
    ) -> FreeRtosBaseType;
    pub fn freertos_rs_timer_start_from_isr(
        timer: FreeRtosTimerHandle,
        xHigherPriorityTaskWoken: FreeRtosBaseTypeMutPtr,
    ) -> FreeRtosBaseType;
    pub fn freertos_rs_timer_stop(
        timer: FreeRtosTimerHandle,
        block_time: FreeRtosTickType,
    ) -> FreeRtosBaseType;
    pub fn freertos_rs_timer_delete(
        timer: FreeRtosTimerHandle,
        block_time: FreeRtosTickType,
    ) -> FreeRtosBaseType;
    pub fn freertos_rs_timer_change_period(
        timer: FreeRtosTimerHandle,
        block_time: FreeRtosTickType,
        new_period: FreeRtosTickType,
    ) -> FreeRtosBaseType;
    pub fn freertos_rs_timer_get_id(timer: FreeRtosTimerHandle) -> FreeRtosVoidPtr;

    pub fn freertos_rs_enter_critical();
    pub fn freertos_rs_exit_critical();
}
