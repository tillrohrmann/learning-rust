use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool {
    workers: Vec<Worker>
}

struct Worker {
    id: u32,
    handle: JoinHandle<()>,
}

struct Job {}

impl Worker {
    fn new(id: u32) -> Worker {
        Worker {
            id,
            handle: thread::spawn(|| {}),
        }
    }
}

impl ThreadPool {
    /// Create new thread pool.
    ///
    /// The size of the thread pool.
    ///
    /// # Panics
    ///
    /// The `new` function panics of the size not larger than 0.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id as u32))
        }

        ThreadPool {
            workers
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}