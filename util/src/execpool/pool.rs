#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    clippy::explicit_iter_loop,
    clippy::future_not_send,
    clippy::use_self,
    clippy::clone_on_ref_ptr
)]

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Priority {
    LowPriority,
    HighPriority,
}

pub fn get_parallelism() -> usize {
    num_cpus::get()
}
use tokio::{select, sync::mpsc};

use parking_lot::Mutex;
use pin_project::{pin_project, pinned_drop};
use std::{pin::Pin, sync::Arc};
use tokio::sync::oneshot::{error::RecvError, Receiver};
use tokio_util::sync::CancellationToken;

use futures::{
    future::{BoxFuture, Shared},
    Future, FutureExt, TryFutureExt,
};

use tracing::warn;

/// Task that can be added to the executor-internal queue.
///
/// Every task within the executor is represented by a [`Job`] that can be polled by the API user.
struct Task {
    fut: Pin<Box<dyn Future<Output = ()> + Send>>,
    cancel: CancellationToken,

    #[allow(dead_code)]
    task_ref: Arc<()>,
}

impl Task {
    /// Run task.
    ///
    /// This runs the payload or cancels if the linked [`Job`] is dropped.
    async fn run(self) {
        tokio::select! {
            _ = self.cancel.cancelled() => (),
            _ = self.fut => (),
        }
    }
}

/// The type of error that is returned from tasks in this module
pub type Error = tokio::sync::oneshot::error::RecvError;

/// Job within the executor.
///
/// Dropping the job will cancel its linked task.
#[pin_project(PinnedDrop)]
#[derive(Debug)]
pub struct Job<T> {
    cancel: CancellationToken,
    detached: bool,
    #[pin]
    rx: Receiver<T>,
}

impl<T> Job<T> {
    /// Detached job so dropping it does not cancel it.
    ///
    /// You must ensure that this task eventually finishes, otherwise [`DedicatedExecutor::join`] may never return!
    pub fn detach(mut self) {
        // cannot destructure `Self` because we implement `Drop`, so we use a flag instead to prevent cancelation.
        self.detached = true;
    }
}

impl<T> Future for Job<T> {
    type Output = Result<T, Error>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        this.rx.poll(cx)
    }
}

#[pinned_drop]
impl<T> PinnedDrop for Job<T> {
    fn drop(self: Pin<&mut Self>) {
        if !self.detached {
            self.cancel.cancel();
        }
    }
}

/// Runs futures (and any `tasks` that are `tokio::task::spawned` by
/// them) on a separate tokio Executor
#[derive(Clone)]
pub struct DedicatedExecutor {
    state: Arc<Mutex<State>>,
}

/// Runs futures (and any `tasks` that are `tokio::task::spawned` by
/// them) on a separate tokio Executor
struct State {
    /// Channel for requests -- the dedicated executor takes requests
    /// from here and runs them.
    ///
    /// This is `None` if we triggered shutdown.
    high_pri_requests: Option<mpsc::Sender<Task>>,
    low_pri_requests: Option<mpsc::Sender<Task>>,

    /// Receiver side indicating that shutdown is complete.
    completed_shutdown: Shared<BoxFuture<'static, Result<(), Arc<RecvError>>>>,

    /// Task counter (uses Arc strong count).
    task_refs: Arc<()>,

    /// The inner thread that can be used to join during drop.
    thread: Option<std::thread::JoinHandle<()>>,
    shutdown_send: mpsc::Sender<()>,
}

// IMPORTANT: Implement `Drop` for `State`, NOT for `DedicatedExecutor`, because the executor can be cloned and clones
// share their inner state.
impl Drop for State {
    fn drop(&mut self) {
        if self.low_pri_requests.is_some() {
            warn!("DedicatedExecutor dropped without calling shutdown()");
            self.low_pri_requests = None;
        }

        if self.high_pri_requests.is_some() {
            warn!("DedicatedExecutor dropped without calling shutdown()");
            self.high_pri_requests = None;
        }

        // do NOT poll the shared future if we are panicking due to https://github.com/rust-lang/futures-rs/issues/2575
        if !std::thread::panicking() && self.completed_shutdown.clone().now_or_never().is_none() {
            warn!("DedicatedExecutor dropped without waiting for worker termination",);
        }

        // join thread but don't care about the results
        self.thread.take().expect("not dropped yet").join().ok();
    }
}

impl std::fmt::Debug for DedicatedExecutor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Avoid taking the mutex in debug formatting
        write!(f, "DedicatedExecutor")
    }
}

impl DedicatedExecutor {
    /// Creates a new `DedicatedExecutor` with a dedicated tokio
    /// executor that is separate from the threadpool created via
    /// `[tokio::main]` or similar.
    ///
    /// The worker thread priority is set to low so that such tasks do
    /// not starve other more important tasks (such as answering health checks)
    ///
    /// Follows the example from to stack overflow and spawns a new
    /// thread to install a Tokio runtime "context"
    /// <https://stackoverflow.com/questions/62536566>
    ///
    /// If you try to do this from a async context you see something like
    /// thread 'plan::stringset::tests::test_builder_plan' panicked at 'Cannot
    /// drop a runtime in a context where blocking is not allowed. This
    /// happens when a runtime is dropped from within an asynchronous
    /// context.', .../tokio-1.4.0/src/runtime/blocking/shutdown.rs:51:21
    pub fn new(thread_name: &str, num_threads: Option<usize>) -> Self {
        let thread_name = thread_name.to_string();

        let (high_pri_tx_tasks, mut high_pri_rx_tasks) =
            mpsc::channel::<Task>(get_parallelism() * 2);
        let (low_pri_tx_tasks, mut low_pri_rx_tasks) = mpsc::channel::<Task>(get_parallelism() * 2);
        let (shutdown_send, mut shutdown_recv) = mpsc::channel(1);

        let (tx_shutdown, rx_shutdown) = tokio::sync::oneshot::channel();

        let thread = std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .thread_name(&thread_name)
                .worker_threads(num_threads.unwrap_or(get_parallelism()))
                .build()
                .expect("Creating tokio runtime");

            runtime.block_on(async move {
                // Dropping the tokio runtime only waits for tasks to yield not to complete
                //
                // We therefore use a RwLock to wait for tasks to complete
                let join = Arc::new(tokio::sync::RwLock::new(()));
                loop {
                    select! {
                    Some(task) = high_pri_rx_tasks.recv() => {
                        let join = Arc::clone(&join);
                        let handle = join.read_owned().await;
                        tokio::task::spawn(async move {
                            task.run().await;
                            std::mem::drop(handle);
                        });
                        continue;
                    }
                    Some(task) = low_pri_rx_tasks.recv() => {
                        let join = Arc::clone(&join);
                        let handle = join.read_owned().await;
                        tokio::task::spawn(async move {
                            task.run().await;
                            std::mem::drop(handle);
                        });
                    }
                    _ = shutdown_recv.recv() => {break;},
                    }
                }

                // Wait for all tasks to finish
                join.write().await;

                // signal shutdown, but it's OK if the other side is gone
                tx_shutdown.send(()).ok();
            })
        });

        let state = State {
            high_pri_requests: Some(high_pri_tx_tasks),
            low_pri_requests: Some(low_pri_tx_tasks),
            task_refs: Arc::new(()),
            completed_shutdown: rx_shutdown.map_err(Arc::new).boxed().shared(),
            thread: Some(thread),
            shutdown_send,
        };

        Self {
            state: Arc::new(Mutex::new(state)),
        }
    }

    /// Runs the specified Future (and any tasks it spawns) on the
    /// `DedicatedExecutor`.
    ///
    /// Currently all tasks are added to the tokio executor
    /// immediately and compete for the threadpool's resources.
    pub fn spawn<T>(&self, task: T, priority: Priority) -> Job<T::Output>
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
    {
        let (tx, rx) = tokio::sync::oneshot::channel();

        let fut = Box::pin(async move {
            let task_output = task.await;
            if tx.send(task_output).is_err() {
                warn!("Spawned task output ignored: receiver dropped")
            }
        });
        let cancel = CancellationToken::new();
        let mut state = self.state.lock();
        let task = Task {
            fut,
            cancel: cancel.clone(),
            task_ref: Arc::clone(&state.task_refs),
        };
        if let Some(requests) = &mut state.high_pri_requests && priority == Priority::HighPriority{
            // would fail if someone has started shutdown
            requests.try_send(task).ok();
        } else if let Some(requests) = &mut state.low_pri_requests && priority == Priority::LowPriority{
            // would fail if someone has started shutdown
            requests.try_send(task).ok();
        } else {
            warn!("tried to schedule task on an executor that was shutdown");
        }

        Job {
            rx,
            cancel,
            detached: false,
        }
    }

    /// Number of currently active tasks.
    pub fn tasks(&self) -> usize {
        let state = self.state.lock();

        // the strong count is always `1 + jobs` because of the Arc we hold within Self
        Arc::strong_count(&state.task_refs).saturating_sub(1)
    }

    /// signals shutdown of this executor and any Clones
    pub fn shutdown(&self) {
        // hang up the channel which will cause the dedicated thread
        // to quit
        let mut state = self.state.lock();
        state.shutdown_send.try_send(()).ok();
        state.high_pri_requests = None;
        state.low_pri_requests = None;
    }

    /// Stops all subsequent task executions, and waits for the worker
    /// thread to complete. Note this will shutdown all clones of this
    /// `DedicatedExecutor` as well.
    ///
    /// Only the first all to `join` will actually wait for the
    /// executing thread to complete. All other calls to join will
    /// complete immediately.
    ///
    /// # Panic / Drop
    /// [`DedicatedExecutor`] implements shutdown on [`Drop`]. You should just use this behavior and NOT call
    /// [`join`](Self::join) manually during [`Drop`] or panics because this might lead to another panic, see
    /// <https://github.com/rust-lang/futures-rs/issues/2575>.
    pub async fn join(&self) {
        self.shutdown();

        // get handle mutex is held
        let handle = {
            let state = self.state.lock();
            state.completed_shutdown.clone()
        };

        // wait for completion while not holding the mutex to avoid
        // deadlocks
        handle.await.expect("Thread died?")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        sync::{Arc, Barrier},
        time::Duration,
    };
    use tokio::sync::Barrier as AsyncBarrier;

    #[tokio::test]
    async fn basic() {
        let barrier = Arc::new(Barrier::new(2));

        let exec = DedicatedExecutor::new("Test DedicatedExecutor", 1);
        let dedicated_task = exec.spawn(do_work(42, Arc::clone(&barrier)), Priority::LowPriority);

        // Note the dedicated task will never complete if it runs on
        // the main tokio thread (as this test is not using the
        // 'multithreaded' version of the executor and the call to
        // barrier.wait actually blocks the tokio thread)
        barrier.wait();

        // should be able to get the result
        assert_eq!(dedicated_task.await.unwrap(), 42);

        exec.join().await;
    }

    #[tokio::test]
    async fn basic_clone() {
        let barrier = Arc::new(Barrier::new(2));
        let exec = DedicatedExecutor::new("Test DedicatedExecutor", 1);
        // Run task on clone should work fine
        let dedicated_task = exec
            .clone()
            .spawn(do_work(42, Arc::clone(&barrier)), Priority::LowPriority);
        barrier.wait();
        assert_eq!(dedicated_task.await.unwrap(), 42);

        exec.join().await;
    }

    #[tokio::test]
    async fn drop_clone() {
        let barrier = Arc::new(Barrier::new(2));
        let exec = DedicatedExecutor::new("Test DedicatedExecutor", 1);

        drop(exec.clone());

        let task = exec.spawn(do_work(42, Arc::clone(&barrier)), Priority::LowPriority);
        barrier.wait();
        assert_eq!(task.await.unwrap(), 42);

        exec.join().await;
    }

    #[tokio::test]
    #[should_panic(expected = "foo")]
    async fn just_panic() {
        struct S(DedicatedExecutor);

        impl Drop for S {
            fn drop(&mut self) {
                self.0.join().now_or_never();
            }
        }

        let exec = DedicatedExecutor::new("Test DedicatedExecutor", 1);
        let _s = S(exec);

        // this must not lead to a double-panic and SIGILL
        panic!("foo")
    }

    #[tokio::test]
    async fn multi_task() {
        let barrier = Arc::new(Barrier::new(3));

        // make an executor with two threads
        let exec = DedicatedExecutor::new("Test DedicatedExecutor", 2);
        let dedicated_task1 = exec.spawn(do_work(11, Arc::clone(&barrier)), Priority::LowPriority);
        let dedicated_task2 = exec.spawn(do_work(42, Arc::clone(&barrier)), Priority::HighPriority);

        // block main thread until completion of other two tasks
        barrier.wait();

        // should be able to get the result
        assert_eq!(dedicated_task1.await.unwrap(), 11);
        assert_eq!(dedicated_task2.await.unwrap(), 42);

        exec.join().await;
    }

    #[tokio::test]
    async fn tokio_spawn() {
        let exec = DedicatedExecutor::new("Test DedicatedExecutor", 2);

        // spawn a task that spawns to other tasks and ensure they run on the dedicated
        // executor
        let dedicated_task = exec.spawn(
            async move {
                // spawn separate tasks
                let t1 = tokio::task::spawn(async {
                    assert_eq!(
                        std::thread::current().name(),
                        Some("Test DedicatedExecutor")
                    );
                    25usize
                });
                t1.await.unwrap()
            },
            Priority::LowPriority,
        );

        // Validate the inner task ran to completion (aka it did not panic)
        assert_eq!(dedicated_task.await.unwrap(), 25);

        exec.join().await;
    }

    #[tokio::test]
    async fn panic_on_executor() {
        let exec = DedicatedExecutor::new("Test DedicatedExecutor", 1);
        let dedicated_task = exec.spawn(
            async move {
                if true {
                    panic!("At the disco, on the dedicated task scheduler");
                } else {
                    42
                }
            },
            Priority::LowPriority,
        );

        // should not be able to get the result
        dedicated_task.await.unwrap_err();

        exec.join().await;
    }

    #[tokio::test]
    async fn executor_shutdown_while_task_running() {
        let barrier = Arc::new(Barrier::new(2));
        let captured = Arc::clone(&barrier);

        let exec = DedicatedExecutor::new("Test DedicatedExecutor", 1);
        let dedicated_task = exec.spawn(
            async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
                do_work(42, captured).await
            },
            Priority::LowPriority,
        );

        exec.shutdown();
        // block main thread until completion of the outstanding task
        barrier.wait();

        // task should complete successfully
        assert_eq!(dedicated_task.await.unwrap(), 42);

        exec.join().await;
    }

    #[tokio::test]
    async fn executor_submit_task_after_shutdown() {
        let exec = DedicatedExecutor::new("Test DedicatedExecutor", 1);

        // Simulate trying to submit tasks once executor has shutdown
        exec.shutdown();
        let dedicated_task = exec.spawn(async { 11 }, Priority::LowPriority);

        // task should complete, but return an error
        dedicated_task.await.unwrap_err();

        exec.join().await;
    }

    #[tokio::test]
    async fn executor_submit_task_after_clone_shutdown() {
        let exec = DedicatedExecutor::new("Test DedicatedExecutor", 1);

        // shutdown the clone (but not the exec)
        exec.clone().join().await;

        // Simulate trying to submit tasks once executor has shutdown
        let dedicated_task = exec.spawn(async { 11 }, Priority::LowPriority);

        // task should complete, but return an error
        dedicated_task.await.unwrap_err();

        exec.join().await;
    }

    #[tokio::test]
    async fn executor_join() {
        let exec = DedicatedExecutor::new("Test DedicatedExecutor", 1);
        // test it doesn't hang
        exec.join().await;
    }

    #[tokio::test]
    async fn executor_join2() {
        let exec = DedicatedExecutor::new("Test DedicatedExecutor", 1);
        // test it doesn't hang
        exec.join().await;
        exec.join().await;
    }

    #[tokio::test]
    #[allow(clippy::redundant_clone)]
    async fn executor_clone_join() {
        let exec = DedicatedExecutor::new("Test DedicatedExecutor", 1);
        // test it doesn't hang
        exec.clone().join().await;
        exec.clone().join().await;
        exec.join().await;
    }

    #[tokio::test]
    async fn drop_receiver() {
        // create empty executor
        let exec = DedicatedExecutor::new("Test DedicatedExecutor", 1);
        assert_eq!(exec.tasks(), 0);

        // create first blocked task
        let barrier1 = Arc::new(AsyncBarrier::new(2));
        let dedicated_task1 = exec.spawn(
            do_work_async(11, Arc::clone(&barrier1)),
            Priority::LowPriority,
        );
        assert_eq!(exec.tasks(), 1);

        // create second blocked task
        let barrier2 = Arc::new(AsyncBarrier::new(2));
        let dedicated_task2 = exec.spawn(
            do_work_async(22, Arc::clone(&barrier2)),
            Priority::LowPriority,
        );
        assert_eq!(exec.tasks(), 2);

        // cancel task
        drop(dedicated_task1);

        // cancelation might take a short while
        wait_for_tasks(&exec, 1).await;

        // unblock other task
        barrier2.wait().await;
        assert_eq!(dedicated_task2.await.unwrap(), 22);
        wait_for_tasks(&exec, 0).await;
        assert_eq!(exec.tasks(), 0);

        exec.join().await;
    }

    #[tokio::test]
    async fn detach_receiver() {
        // create empty executor
        let exec = DedicatedExecutor::new("Test DedicatedExecutor", 1);
        assert_eq!(exec.tasks(), 0);

        // create first task
        // `detach()` consumes the task but doesn't abort the task (in contrast to `drop`). We'll proof the that the
        // task is still running by linking it to a 2nd task using a barrier with size 3 (two tasks plus the main thread).
        let barrier = Arc::new(AsyncBarrier::new(3));
        let dedicated_task = exec.spawn(
            do_work_async(11, Arc::clone(&barrier)),
            Priority::LowPriority,
        );
        dedicated_task.detach();
        assert_eq!(exec.tasks(), 1);

        // create second task
        let dedicated_task = exec.spawn(
            do_work_async(22, Arc::clone(&barrier)),
            Priority::HighPriority,
        );
        assert_eq!(exec.tasks(), 2);

        // wait a bit just to make sure that our tasks doesn't get dropped
        tokio::time::sleep(Duration::from_millis(10)).await;
        assert_eq!(exec.tasks(), 2);

        // tasks should be unblocked because they both wait on the same barrier
        // unblock tasks
        barrier.wait().await;
        wait_for_tasks(&exec, 0).await;
        let result = dedicated_task.await.unwrap();
        assert_eq!(result, 22);

        exec.join().await;
    }

    /// Wait for the barrier and then return `result`
    async fn do_work(result: usize, barrier: Arc<Barrier>) -> usize {
        barrier.wait();
        result
    }

    /// Wait for the barrier and then return `result`
    async fn do_work_async(result: usize, barrier: Arc<AsyncBarrier>) -> usize {
        barrier.wait().await;
        result
    }

    // waits for up to 1 sec for the correct number of tasks
    async fn wait_for_tasks(exec: &DedicatedExecutor, num: usize) {
        tokio::time::timeout(Duration::from_secs(1), async {
            loop {
                if dbg!(exec.tasks()) == num {
                    return;
                }
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
        })
        .await
        .expect("Did not find expected num tasks within a second")
    }
}
