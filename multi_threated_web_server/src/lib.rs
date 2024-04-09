use std::{
    fmt,
    sync::{mpsc, Arc, Mutex},
    thread,
};

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // Avoid `while` b/c it does not drop temporary values until end of block
            // This would cause all workers to wait on sleep which locks receiver for 5s
            // With `let` all temp values in right hand size are dropped immediately, thus unlocking receiver
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} shutting down.");
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
pub enum PoolCreationError {
    SizeTooSmall,
    SizeTooLarge,
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PoolCreationError::SizeTooSmall => write!(f, "Pool size is too small."),
            PoolCreationError::SizeTooLarge => write!(f, "Pool size is too large."),
        }
    }
}

impl ThreadPool {
    /// Creates a new ThreadPool
    ///
    /// The size is the # of threads in the pool.
    ///
    /// # Errors
    ///
    /// The `build` function will reurn Err() if the 1 > size > 4.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        match size {
            1..=4 => {
                let (sender, receiver) = mpsc::channel();
                let receiver = Arc::new(Mutex::new(receiver));
                let mut workers = Vec::with_capacity(size);

                for id in 0..size {
                    workers.push(Worker::new(id, Arc::clone(&receiver)));
                }
                Ok(ThreadPool {
                    workers,
                    sender: Some(sender),
                })
            }
            0 => Err(PoolCreationError::SizeTooSmall),
            _ => Err(PoolCreationError::SizeTooLarge),
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
