use super::*;
use futures::Future;

pub struct Backlog {
    executor: DedicatedExecutor,
    priority: Priority,
}

impl Backlog {
    pub fn new(exec: DedicatedExecutor, priority: Priority) -> Self {
        return Self {
            executor: exec,
            priority,
        };
    }

    pub fn enqueue<T>(&self, task: T) -> Job<T::Output>
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
    {
        self.executor.spawn(task, self.priority)
    }
}
