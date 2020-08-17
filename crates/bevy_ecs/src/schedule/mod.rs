
/// This can be added as an app resource to control the global `rayon::ThreadPool` used by ecs.
// Dev internal note: We cannot directly expose a ThreadPoolBuilder here as it does not implement Send and Sync.
#[derive(Debug, Default, Clone)]
pub struct ParallelExecutorOptions {
    /// If some value, we'll set up the thread pool to use at most n threads. See `rayon::ThreadPoolBuilder::num_threads`.
    num_threads: Option<usize>,
    /// If some value, we'll set up the thread pool's' workers to the given stack size. See `rayon::ThreadPoolBuilder::stack_size`.
    stack_size: Option<usize>,
    // TODO: Do we also need/want to expose other features (*_handler, etc.)
}

impl ParallelExecutorOptions {
    /// Creates a new ParallelExecutorOptions instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the num_threads option, using the builder pattern
    pub fn with_num_threads(mut self, num_threads: Option<usize>) -> Self {
        self.num_threads = num_threads;
        self
    }

    /// Sets the stack_size option, using the builder pattern. WARNING: Only use this if you know what you're doing,
    /// otherwise your application may run into stability and performance issues.
    pub fn with_stack_size(mut self, stack_size: Option<usize>) -> Self {
        self.stack_size = stack_size;
        self
    }

    #[cfg(feature = "rayon_executor")]
    /// Creates a new ThreadPoolBuilder based on the current options.
    pub(crate) fn create_builder(&self) -> rayon::ThreadPoolBuilder {
        let mut builder = rayon::ThreadPoolBuilder::new();

        if let Some(num_threads) = self.num_threads {
            builder = builder.num_threads(num_threads);
        }

        if let Some(stack_size) = self.stack_size {
            builder = builder.stack_size(stack_size);
        }

        builder
    }
}

#[cfg(feature = "rayon_executor")]
mod rayon_executor;
#[cfg(feature = "rayon_executor")]
pub use rayon_executor::RayonExecutor;
#[cfg(feature = "rayon_executor")]
pub type DefaultExecutor = RayonExecutor;


#[cfg(feature = "simple_executor")]
mod simple_executor;
#[cfg(feature = "simple_executor")]
pub use simple_executor::SimpleExecutor;
#[cfg(feature = "simple_executor")]
pub type DefaultExecutor = SimpleExecutor;

#[cfg(feature = "multitask_executor")]
mod multitask_executor;
#[cfg(feature = "multitask_executor")]
pub use multitask_executor::MultitaskExecutor;
#[cfg(feature = "multitask_executor")]
pub type DefaultExecutor = MultitaskExecutor;

mod schedule;
pub use schedule::*;
