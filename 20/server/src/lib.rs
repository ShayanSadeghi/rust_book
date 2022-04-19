use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ThreadPool {
    // create a channel and hold on to the sending side of the channel
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

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
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    // each worker will hold on to the receiving side of the channel
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // we 'lock' on the receiver to acquire the mutex
            // then call 'unwrap' to panic on any errors
            // we call 'recv' to receive a Job from the channel
            // then an 'unwrap' moves past any errors
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job executing.", id);
            job();
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
