use crate::base::*;
use crate::isr::*;
use crate::prelude::v1::*;
use crate::shim::*;
use crate::units::*;
use crate::utils::*;

unsafe impl Send for Task {}

/// Handle for a FreeRTOS task
#[derive(Debug, Clone)]
pub struct Task {
    task_handle: FreeRtosTaskHandle,
}

/// Task's execution priority. Low priority numbers denote low priority tasks.
#[derive(Debug, Copy, Clone)]
pub struct TaskPriority(pub u8);

/// Notification to be sent to a task.
#[derive(Debug, Copy, Clone)]
pub enum TaskNotification {
    /// Send the event, unblock the task, the task's notification value isn't changed.
    NoAction,
    /// Perform a logical or with the task's notification value.
    SetBits(u32),
    /// Increment the task's notification value by one.
    Increment,
    /// Set the task's notification value to this value.
    OverwriteValue(u32),
    /// Try to set the task's notification value to this value. Succeeds
    /// only if the task has no pending notifications. Otherwise, the
    /// notification call will fail.
    SetValue(u32),
}

impl TaskNotification {
    fn to_freertos(&self) -> (u32, u8) {
        match *self {
            TaskNotification::NoAction => (0, 0),
            TaskNotification::SetBits(v) => (v, 1),
            TaskNotification::Increment => (0, 2),
            TaskNotification::OverwriteValue(v) => (v, 3),
            TaskNotification::SetValue(v) => (v, 4),
        }
    }
}

impl TaskPriority {
    fn to_freertos(&self) -> FreeRtosUBaseType {
        self.0 as FreeRtosUBaseType
    }
}

/// Helper for spawning a new task. Instantiate with [`Task::new()`].
///
/// [`Task::new()`]: struct.Task.html#method.new
pub struct TaskBuilder {
    task_name: String,
    task_stack_size: u16,
    task_priority: TaskPriority,
}

impl TaskBuilder {
    /// Set the task's name.
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.task_name = name.into();
        self
    }

    /// Set the stack size, in words.
    pub fn stack_size(&mut self, stack_size: u16) -> &mut Self {
        self.task_stack_size = stack_size;
        self
    }

    /// Set the task's priority.
    pub fn priority(&mut self, priority: TaskPriority) -> &mut Self {
        self.task_priority = priority;
        self
    }

    /// Start a new task that can't return a value.
    pub fn start<F>(&self, func: F) -> Result<Task, FreeRtosError>
    where
        F: FnOnce(Task) -> (),
        F: Send + 'static,
    {
        Task::spawn(
            &self.task_name,
            self.task_stack_size,
            self.task_priority,
            func,
        )
    }
}

impl Task {
    /// Prepare a builder object for the new task.
    pub fn new() -> TaskBuilder {
        TaskBuilder {
            task_name: "rust_task".into(),
            task_stack_size: 1024,
            task_priority: TaskPriority(1),
        }
    }

    #[inline]
    pub unsafe fn from_raw_handle(handle: FreeRtosTaskHandle) -> Self {
        Self { task_handle: handle }
    }

    pub fn suspend_all() {
      unsafe {
          freertos_rs_vTaskSuspendAll();
      }
    }

    pub fn resume_all() {
        unsafe {
            freertos_rs_xTaskResumeAll();
        }
    }

    unsafe fn spawn_inner<'a>(
        f: Box<dyn FnOnce(Task)>,
        name: &str,
        stack_size: u16,
        priority: TaskPriority,
    ) -> Result<Task, FreeRtosError> {
        let f = Box::new(f);
        let param_ptr = &*f as *const _ as *mut _;

        let (success, task_handle) = {
            let name = name.as_bytes();
            let name_len = name.len();
            let mut task_handle = core::ptr::null();

            let ret = freertos_rs_spawn_task(
                thread_start,
                param_ptr,
                name.as_ptr(),
                name_len as u8,
                stack_size,
                priority.to_freertos(),
                &mut task_handle,
            );

            (ret == 0, task_handle)
        };

        if success {
            mem::forget(f);
        } else {
            return Err(FreeRtosError::OutOfMemory);
        }

        use core::ffi::c_void;
        extern "C" fn thread_start(main: *mut c_void) -> *mut c_void {
            unsafe {
                {
                    let b = Box::from_raw(main as *mut Box<dyn FnOnce(Task)>);
                    b(Task {
                        task_handle: freertos_rs_get_current_task(),
                    });
                }

                #[cfg(feature = "delete_task")]
                freertos_rs_delete_task(0 as *const _);
            }

            #[cfg(feature = "delete_task")]
            return 0 as *mut _;
            #[cfg(not(feature = "delete_task"))]
            panic!("Not allowed to quit the task!");
        }

        Ok(Task { task_handle })
    }

    fn spawn<F>(
        name: &str,
        stack_size: u16,
        priority: TaskPriority,
        f: F,
    ) -> Result<Task, FreeRtosError>
    where
        F: FnOnce(Task) -> (),
        F: Send + 'static,
    {
        unsafe {
            return Task::spawn_inner(Box::new(f), name, stack_size, priority);
        }
    }

    /// Get the name of the current task.
    pub fn get_name(&self) -> Result<String, ()> {
        unsafe {
            let name_ptr = freertos_rs_task_get_name(self.task_handle);
            let name = str_from_c_string(name_ptr);
            if let Ok(name) = name {
                return Ok(name);
            }

            Err(())
        }
    }

    /// Try to find the task of the current execution context.
    pub fn current() -> Result<Task, FreeRtosError> {
        unsafe {
            let t = freertos_rs_get_current_task();
            if t != 0 as *const _ {
                Ok(Task { task_handle: t })
            } else {
                Err(FreeRtosError::TaskNotFound)
            }
        }
    }

    /// Forcibly set the notification value for this task.
    pub fn set_notification_value(&self, val: u32) {
        self.notify(TaskNotification::OverwriteValue(val))
    }

    /// Notify this task.
    pub fn notify(&self, notification: TaskNotification) {
        unsafe {
            let n = notification.to_freertos();
            freertos_rs_task_notify(self.task_handle, n.0, n.1);
        }
    }

    /// Notify this task from an interrupt.
    pub fn notify_from_isr(
        &self,
        context: &InterruptContext,
        notification: TaskNotification,
    ) -> Result<(), FreeRtosError> {
        unsafe {
            let n = notification.to_freertos();
            let t = freertos_rs_task_notify_isr(
                self.task_handle,
                n.0,
                n.1,
                context.get_task_field_mut(),
            );
            if t != 0 {
                Err(FreeRtosError::QueueFull)
            } else {
                Ok(())
            }
        }
    }

    /// Wait for a notification to be posted.
    pub fn wait_for_notification<D: DurationTicks>(
        &self,
        clear_bits_enter: u32,
        clear_bits_exit: u32,
        wait_for: D,
    ) -> Result<u32, FreeRtosError> {
        let mut val = 0;
        let r = unsafe {
            freertos_rs_task_notify_wait(
                clear_bits_enter,
                clear_bits_exit,
                &mut val as *mut _,
                wait_for.to_ticks(),
            )
        };

        if r == 0 {
            Ok(val)
        } else {
            Err(FreeRtosError::Timeout)
        }
    }

    /// Get the minimum amount of stack that was ever left on this task.
    pub fn get_stack_high_water_mark(&self) -> u32 {
        unsafe { freertos_rs_get_stack_high_water_mark(self.task_handle) as u32 }
    }

    pub fn get_id(&self) -> Result<FreeRtosBaseType, FreeRtosError> {
        let task_id = unsafe { freertos_rs_uxTaskGetTaskNumber(self.task_handle) };
        if task_id == 0 {
            Err(FreeRtosError::TaskNotFound)
        } else {
            Ok(task_id)
        }
    }

    pub fn set_id(&mut self, value: FreeRtosUBaseType) {
        unsafe { freertos_rs_vTaskSetTaskNumber(self.task_handle, value) };
    }
}

/// Helper methods to be performed on the task that is currently executing.
pub struct CurrentTask;

impl CurrentTask {
    /// Delay the execution of the current task.
    pub fn delay<D: DurationTicks>(delay: D) {
        unsafe {
            freertos_rs_vTaskDelay(delay.to_ticks());
        }
    }

    pub fn suspend() {
        unsafe {
            freertos_rs_suspend_task(0 as FreeRtosTaskHandle)
        }
    }

    /// Take the notification and either clear the notification value or decrement it by one.
    pub fn take_notification<D: DurationTicks>(clear: bool, wait_for: D) -> u32 {
        unsafe { freertos_rs_task_notify_take(if clear { 1 } else { 0 }, wait_for.to_ticks()) }
    }

    /// Get the minimum amount of stack that was ever left on the current task.
    pub fn get_stack_high_water_mark() -> u32 {
        unsafe { freertos_rs_get_stack_high_water_mark(0 as FreeRtosTaskHandle) as u32 }
    }
}

#[derive(Debug)]
pub struct FreeRtosSystemState {
    pub tasks: Vec<FreeRtosTaskStatus>,
    pub total_run_time: u32,
}

impl fmt::Display for FreeRtosSystemState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str("FreeRTOS tasks\r\n")?;

        write!(fmt, "{id: <6} | {name: <16} | {state: <9} | {priority: <8} | {stack: >10} | {cpu_abs: >10} | {cpu_rel: >4}\r\n",
               id = "ID",
               name = "Name",
               state = "State",
               priority = "Priority",
               stack = "Stack left",
               cpu_abs = "CPU",
               cpu_rel = "%"
        )?;

        for task in &self.tasks {
            write!(fmt, "{id: <6} | {name: <16} | {state: <9} | {priority: <8} | {stack: >10} | {cpu_abs: >10} | {cpu_rel: >4}\r\n",
                   id = task.task_number,
                   name = task.name,
                   state = format!("{:?}", task.task_state),
                   priority = task.current_priority.0,
                   stack = task.stack_high_water_mark,
                   cpu_abs = task.run_time_counter,
                   cpu_rel = if self.total_run_time > 0 && task.run_time_counter <= self.total_run_time {
                       let p = (((task.run_time_counter as u64) * 100) / self.total_run_time as u64) as u32;
                       let ps = if p == 0 && task.run_time_counter > 0 {
                           "<1".to_string()
                       } else {
                           p.to_string()
                       };
                       format!("{: >3}%", ps)
                   } else {
                       "-".to_string()
                   }
            )?;
        }

        if self.total_run_time > 0 {
            write!(fmt, "Total run time: {}\r\n", self.total_run_time)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct FreeRtosTaskStatus {
    pub task: Task,
    pub name: String,
    pub task_number: FreeRtosUBaseType,
    pub task_state: FreeRtosTaskState,
    pub current_priority: TaskPriority,
    pub base_priority: TaskPriority,
    pub run_time_counter: FreeRtosUnsignedLong,
    pub stack_high_water_mark: FreeRtosUnsignedShort,
}

pub struct FreeRtosUtils;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FreeRtosSchedulerState {
  Suspended,
  NotStarted,
  Running
}

impl FreeRtosUtils {
    // Should only be used for testing purpose!
    pub fn invoke_assert() {
        unsafe {
            freertos_rs_invoke_configASSERT();
        }
    }
    pub fn start_scheduler() -> ! {
        unsafe {
            freertos_rs_vTaskStartScheduler();
        }
    }

    pub fn scheduler_state() -> FreeRtosSchedulerState {
      unsafe {
        match freertos_rt_xTaskGetSchedulerState() {
          0 => FreeRtosSchedulerState::Suspended,
          1 => FreeRtosSchedulerState::NotStarted,
          2 => FreeRtosSchedulerState::Running,
          _ => unreachable!(),
        }
      }
    }

    pub fn get_tick_count() -> FreeRtosTickType {
        unsafe { freertos_rs_xTaskGetTickCount() }
    }

    pub fn get_tick_count_duration() -> Duration {
        Duration::ticks(Self::get_tick_count())
    }

    pub fn get_number_of_tasks() -> usize {
        unsafe { freertos_rs_get_number_of_tasks() as usize }
    }

    pub fn get_all_tasks(tasks_len: Option<usize>) -> FreeRtosSystemState {
        let tasks_len = tasks_len.unwrap_or(Self::get_number_of_tasks());
        let mut tasks = Vec::with_capacity(tasks_len as usize);
        let mut total_run_time = 0;

        unsafe {
            let filled = freertos_rs_get_system_state(
                tasks.as_mut_ptr(),
                tasks_len as FreeRtosUBaseType,
                &mut total_run_time,
            );
            tasks.set_len(filled as usize);
        }

        let tasks = tasks
            .into_iter()
            .map(|t| FreeRtosTaskStatus {
                task: Task {
                    task_handle: t.handle,
                },
                name: unsafe { str_from_c_string(t.task_name) }
                    .unwrap_or_else(|_| String::from("?")),
                task_number: t.task_number,
                task_state: t.task_state,
                current_priority: TaskPriority(t.current_priority as u8),
                base_priority: TaskPriority(t.base_priority as u8),
                run_time_counter: t.run_time_counter,
                stack_high_water_mark: t.stack_high_water_mark,
            })
            .collect();

        FreeRtosSystemState {
            tasks: tasks,
            total_run_time: total_run_time,
        }
    }
}
