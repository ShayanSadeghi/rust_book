use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ThreadPool {
    // create a channel and hold on to the sending side of the channel
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
struct Job; // hold the closures we want to send down the channel

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

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size); // "with_capacity" is like "new", but the space is preallocated

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    // this method will send the job it wants to execute down the sending side of the channel
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker {
    // each worker will hold on to the receiving side of the channel
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });
        Worker { id, thread }
    }
}
