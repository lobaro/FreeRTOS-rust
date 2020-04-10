use crate::prelude::v1::*;
use crate::base::*;
use crate::task::*;
use crate::mutex::*;
use crate::queue::*;
use crate::units::*;

pub trait ComputeTaskBuilder {
    fn compute<F, R>(&self, func: F) -> Result<ComputeTask<R>, FreeRtosError>
        where F: FnOnce() -> R,
              F: Send + 'static,
              R: Sync + Send + 'static;
}

impl ComputeTaskBuilder for TaskBuilder {
    #[cfg(target_os="none")]
    /// Spawn a task that can post a return value to the outside.
    fn compute<F, R>(&self, func: F) -> Result<ComputeTask<R>, FreeRtosError>
        where F: FnOnce() -> R,
              F: Send + 'static,
              R: Sync + Send + 'static
    {

        let (task, result, status) = {
            let result = Arc::new(Mutex::new(None)?);
            let status = Arc::new(Queue::new(1)?);

            let task_result = result.clone();
            let task_status = status.clone();
            let task = self.start(move || {
                {
                    let mut lock = task_result.lock(Duration::infinite()).unwrap();
                    let r = func();
                    *lock = Some(r);
                }
                // release our reference to the mutex, so it can be deconstructed
                drop(task_result);
                task_status.send(ComputeTaskStatus::Finished, Duration::infinite()).unwrap();
            })?;

            (task, result, status)
        };

        Ok(ComputeTask {
            task: task,
            result: result,
            status: status,
            finished: false,
        })
    }

    #[cfg(not(target_os="none"))]
    fn compute<F, R>(&self, func: F) -> Result<ComputeTask<R>, FreeRtosError>
        where F: FnOnce() -> R,
              F: Send + 'static,
              R: Sync + Send + 'static
    {

        let r = func();

        Ok(ComputeTask {
            task: Task::new().start(|| {}).unwrap(),
            result: Arc::new(Mutex::new(Some(r)).unwrap()),
            status: Arc::new(Queue::new(1).unwrap()),
            finished: false,
        })
    }
}

/// A task that can terminate and return its return value. Implemented using an
/// atomically shared mutex.
///
/// Sample usage:
///
/// ```rust
/// # use freertos_rs::*;
/// use freertos_rs::patterns::compute_task::*;
/// let task = Task::new().compute(|| {
/// 	CurrentTask::delay(Duration::ms(100));
/// 	42
/// }).unwrap();
///
/// let result = task.into_result(Duration::ms(1000)).unwrap();
/// # println!("{}", result);
/// ```

pub struct ComputeTask<R> {
    task: Task,
    result: Arc<Mutex<Option<R>>>,
    status: Arc<Queue<ComputeTaskStatus>>,
    finished: bool,
}

#[derive(Debug, Copy, Clone)]
enum ComputeTaskStatus {
    Finished,
}

use core::fmt::Debug;

impl<R: Debug> ComputeTask<R> {
    /// Get the handle of the task that computes the result.
    pub fn get_task(&self) -> &Task {
        &self.task
    }

    /// Wait until the task computes its result. Otherwise, returns a timeout.
    pub fn wait_for_result<D: DurationTicks>(&mut self, max_wait: D) -> Result<(), FreeRtosError> {
        if self.finished == true {
            Ok(())
        } else {
            match self.status.receive(max_wait) {
                Ok(ComputeTaskStatus::Finished) => {
                    self.finished = true;
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
    }

    /// Consume the task and unwrap the computed return value.
    pub fn into_result<D: DurationTicks>(mut self, max_wait: D) -> Result<R, FreeRtosError> {
        self.wait_for_result(max_wait)?;

        if self.finished != true {
            panic!("ComputeTask should be finished!");
        }

        let m = Arc::try_unwrap(self.result)
            .expect("ComputeTask: Arc should have only one reference left!");
        let option_r = m.into_inner();
        let r = option_r.expect("ComputeTask: Result should be a Some(R)!");

        Ok(r)
    }
}
