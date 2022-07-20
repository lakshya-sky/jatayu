use std::{future::Future, pin::Pin};
use tracing::info;

use tokio::{
    runtime::Handle,
    select,
    sync::{
        mpsc::{channel, Receiver, Sender},
        oneshot,
    },
};
type PoolResult<T> = Result<T, Box<dyn std::error::Error + Send>>;

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;
type FuncType = Box<(dyn Fn() -> BoxFuture<()> + Send + Sync)>;

enum Priority {
    LowPriority = 0,
    HighPriority,
    NumPrios,
}

//pub trait ExecutionPool {
//    fn enqueue(&self) -> Result<(), Box<dyn std::error::Error>>;
//    fn shut_down(&self);
//    fn get_parallelism(&self) -> usize;
//}

pub struct Pool {
    num_cpus: usize,
    tokio_runtime: tokio::runtime::Runtime,
    inputs: [Sender<FuncType>; Priority::NumPrios as usize],
    shutdown: oneshot::Sender<()>,
}

impl Pool {
    pub fn new() -> Self {
        let num_cpus = num_cpus::get();
        let (high_pri_tx, high_pri_rx) = channel(1024);
        let (low_pri_tx, low_pri_rx) = channel(1024);
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let inputs = [low_pri_tx, high_pri_tx];
        let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
            .thread_name("exec-pool")
            .worker_threads(num_cpus)
            .enable_all()
            .build()
            .unwrap();
        let runtime_handle = tokio_runtime.handle().clone();
        std::thread::spawn(move || {
            runtime_handle.block_on(start_pool(
                &runtime_handle.clone(),
                high_pri_rx,
                low_pri_rx,
                shutdown_rx,
            ));
        });
        Self {
            tokio_runtime,
            num_cpus,
            inputs,
            shutdown: shutdown_tx,
        }
    }

    async fn enqueue(&self, task: FuncType) -> Result<(), Box<dyn std::error::Error>> {
        self.inputs[0]
            .send(task)
            .await
            .map_err(|_| format!("failed to enqueue task").into())
    }

    pub fn stop(self) {
        let _ = self.shutdown.send(());
        info!("sent shutdown signal");
    }
}

async fn start_pool(
    handle: &Handle,
    mut high_pri_rx: Receiver<FuncType>,
    mut low_pri_rx: Receiver<FuncType>,
    mut shut_down: oneshot::Receiver<()>,
) {
    info!("started pool");
    loop {
        select! {
            Some(high_pri_task) = high_pri_rx.recv() =>{
                handle.spawn((||async move{
                    let _ = high_pri_task().await;
                })());
                continue;
            },
            Some(low_pri_task) = low_pri_rx.recv() => {
                info!("got a low priority task");
                handle.spawn((||async move{
                    let _ = low_pri_task().await;
                })());
            },
             _ = &mut shut_down => {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;
    use futures;
    use tokio::time::sleep;

    #[test]
    fn create_pool() {
        tracing_subscriber::fmt::init();
        let pool = Pool::new();
        futures::executor::block_on(async {
            info!("blocking task");
            for _ in 0..10 {
                let _ = pool
                    .enqueue(Box::new(|| {
                        Box::pin(async {
                            info!("enqueued task");
                            sleep(Duration::from_secs(1)).await;
                        })
                    }))
                    .await;
            }
            info!("finished task");
        });
        pool.stop();
    }
}
