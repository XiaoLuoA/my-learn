use crate::Result;

mod naive_threadpool;
mod rayon_threadpool;
mod shared_queue;

pub use self::naive_threadpool::NaiveThreadPool;
pub use self::rayon_threadpool::RayonThreadPool;
pub use self::shared_queue::SharedQueueThreadPool;

pub trait ThreadPool: Clone + Send + 'static {
    fn new(threads: u32) -> Result<Self>
    where
        Self: Sized;
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}
