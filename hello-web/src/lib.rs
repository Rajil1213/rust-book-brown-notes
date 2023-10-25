pub struct ThreadPool {}

impl ThreadPool {
    pub fn new(num_threads: usize) -> ThreadPool {
        ThreadPool {}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
