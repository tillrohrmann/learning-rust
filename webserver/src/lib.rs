use std::thread;
use std::sync::mpsc;
use std::thread::JoinHandle;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;
use std::sync::Arc;
use std::env::Args;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

pub struct ThreadPool {
    sender: Sender<Job>,
    workers: Vec<Worker>,
}

struct Worker {
    id: usize,
    handle: JoinHandle<()>,
}

type Job = Box<FnBox + Send + 'static>;

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        Worker {
            id,
            handle: thread::spawn(move || {
                loop {
                    let job = receiver.lock().unwrap().recv().unwrap();
                    println!("Worker {} got a job; executing.", id);
                    job.call_box()
                }
            }),
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

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        ThreadPool {
            sender,
            workers,
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}