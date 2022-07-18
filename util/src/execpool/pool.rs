enum Priority {
    LowPriority = 0,
    HighPriority,
    NumPrios,
}
pub trait ExecutionPool {
    fn enqueue(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn shut_down(&self);
    fn get_parallelism(&self) -> usize;
}
pub struct Pool {
    num_cpus: usize,
}

pub fn make_pool() -> Pool {
    todo!();
}
