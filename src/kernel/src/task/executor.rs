//! Task executor
extern crate alloc;

use alloc::{collections::BTreeMap, sync::Arc, task::Wake};
use core::task::{Context, Poll, Waker};

use crossbeam_queue::ArrayQueue;

use super::{Task, TaskId};

/// Task executor that drives tasks to completion
pub struct Executor {
    /// Binary tree of tasks
    tasks: BTreeMap<TaskId, Task>,
    /// Queue of tasks ready to run
    task_queue: Arc<ArrayQueue<TaskId>>,
    /// Cache of wakers for tasks
    waker_cache: BTreeMap<TaskId, Waker>,
}

impl Executor {
    /// Create a new executor
    pub fn new() -> Self {
        Executor {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(100)),
            waker_cache: BTreeMap::new(),
        }
    }

    /// Spawn a new task
    ///
    /// Adds the provided task to the executor
    ///
    /// # Safety
    /// * panics if the task ID is already in the executor
    /// * panics if the task queue is full
    ///
    /// # Arguments
    /// * `task` - [`Task`] to spawn
    pub fn spawn(&mut self, task: Task) {
        let task_id = task.id;
        if self.tasks.insert(task.id, task).is_some() {
            panic!("task with same ID already in tasks");
        }
        self.task_queue.push(task_id).expect("queue full");
    }

    /// Run the executor
    ///
    /// Continuously runs tasks until there are no more tasks to run
    /// and then sleeps until an interrupt is received
    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();
            self.sleep_if_idle();
        }
    }

    /// Run all tasks that are ready to run
    fn run_ready_tasks(&mut self) {
        // destructure `self` to avoid borrow checker errors
        let Self {
            tasks,
            task_queue,
            waker_cache,
        } = self;

        while let Some(task_id) = task_queue.pop() {
            let task = match tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue, // task no longer exists
            };
            let waker = waker_cache
                .entry(task_id)
                .or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));
            let mut context = Context::from_waker(waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {
                    // task done -> remove it and its cached waker
                    tasks.remove(&task_id);
                    waker_cache.remove(&task_id);
                }
                Poll::Pending => {}
            }
        }
    }

    /// Sleep if there are no tasks to run
    ///
    /// If there are no tasks to run, disable interrupts and halt the CPU until
    /// an interrupt is received.
    fn sleep_if_idle(&self) {
        use x86_64::instructions::interrupts::{self, enable_and_hlt};

        interrupts::disable();
        if self.task_queue.is_empty() {
            enable_and_hlt();
        } else {
            interrupts::enable();
        }
    }
}

/// Waker for a task
struct TaskWaker {
    /// Task ID
    task_id: TaskId,
    /// Task queue
    task_queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
    /// Create a new task waker
    #[allow(clippy::new_ret_no_self)]
    fn new(task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(TaskWaker {
            task_id,
            task_queue,
        }))
    }

    /// Wake the task
    fn wake_task(&self) {
        self.task_queue.push(self.task_id).expect("task_queue full");
    }
}

impl Wake for TaskWaker {
    /// Wake the task
    fn wake(self: Arc<Self>) {
        self.wake_task();
    }

    /// Wake the task by reference
    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task();
    }
}
