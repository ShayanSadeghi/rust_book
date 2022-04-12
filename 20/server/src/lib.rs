use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0); // usize can accept non-negative integers, but we don't want 0 too,
                           //so we do this to panic when a user want to create a ThreadPool with 0 thread

        let mut threads = Vec::with_capacity(size); // "with_capacity" is like "new", but the space is preallocated

        for _ in 0..size {
            // create some threads and store them in the vector
        }

        ThreadPool { threads }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
