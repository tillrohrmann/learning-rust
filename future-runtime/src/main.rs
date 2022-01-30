use futures::task::ArcWake;
use futures::{task, FutureExt};
use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::future::Future;
use std::ops::{Add, Deref};
use std::pin::Pin;
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

fn main() {
    let mut result = do_something_async();

    let runtime = FutureRuntime::new();

    let mut handle = runtime.run(result);

    handle.join();
}

struct FutureRuntime {
    sender: SyncSender<Task>,
    thread_handle: JoinHandle<()>,
}

impl FutureRuntime {
    fn new() -> FutureRuntime {
        let (task_queue, receiver) = std::sync::mpsc::sync_channel(10);
        let future_processor_task_queue = task_queue.clone();

        let thread_handle = std::thread::spawn(move || {
            let mut processor = FutureProcessor::new(future_processor_task_queue);

            for task in receiver {
                match task {
                    Task::Stop => break,
                    Task::Future(future, sender) => {
                        processor.run_future(future, sender);
                    }
                    Task::Resume(future_key) => processor.resume(future_key),
                }
            }
        });

        FutureRuntime {
            sender: task_queue,
            thread_handle,
        }
    }

    fn run(&self, future: impl Future<Output = ()> + Unpin + Send + 'static) -> Handle {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1);
        let task = Task::Future(Box::new(future), sender);
        self.sender.send(task);

        Handle::new(receiver)
    }
}

struct FutureProcessor {
    counter: usize,
    task_queue: SyncSender<Task>,
    futures: HashMap<usize, (Box<dyn Future<Output = ()> + Unpin>, SyncSender<()>)>,
}

impl FutureProcessor {
    fn new(task_queue: SyncSender<Task>) -> FutureProcessor {
        FutureProcessor {
            counter: 0,
            task_queue,
            futures: HashMap::new(),
        }
    }

    fn run_future(
        &mut self,
        mut future: Box<dyn Future<Output = ()> + Unpin>,
        sender: SyncSender<()>,
    ) {
        self.futures.insert(self.counter, (future, sender));
        self.resume(self.counter);
        self.counter += 1;
    }

    fn resume(&mut self, future_key: usize) {
        if let Some((mut future, sender)) = self.futures.remove(&future_key) {
            if !self.run_future_internally(future_key, &mut future, &sender) {
                self.futures.insert(future_key, (future, sender));
            }
        }
    }

    fn run_future_internally(
        &self,
        key: usize,
        future: &mut Box<dyn Future<Output = ()> + Unpin>,
        sender: &SyncSender<()>,
    ) -> bool {
        let myWaker = MyWaker::new(key, self.task_queue.clone());
        let waker = task::waker(Arc::new(myWaker));
        let mut context = Context::from_waker(&waker);

        match Pin::new(future.as_mut()).poll(&mut context) {
            Poll::Ready(()) => {
                sender.send(()).unwrap();
                true
            }
            Poll::Pending => false, // wait for the future to complete
        }
    }
}

struct MyWaker {
    task_queue: SyncSender<Task>,
    future_key: usize,
}

impl MyWaker {
    fn new(future_key: usize, task_queue: SyncSender<Task>) -> MyWaker {
        MyWaker {
            task_queue,
            future_key,
        }
    }
}

impl ArcWake for MyWaker {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.task_queue.send(Task::Resume(arc_self.future_key));
    }
}

struct Handle {
    receiver: Receiver<()>,
}

impl Handle {
    fn join(&mut self) {
        self.receiver.recv().unwrap();
    }

    fn new(receiver: Receiver<()>) -> Handle {
        Handle { receiver }
    }
}

struct PollingFutureRuntime {
    sender: SyncSender<Task>,
    thread_join_handle: JoinHandle<()>,
}

enum Task {
    Stop,
    Future(Box<dyn Future<Output = ()> + Unpin + Send>, SyncSender<()>),
    Resume(usize),
}

impl PollingFutureRuntime {
    fn new() -> PollingFutureRuntime {
        let (sender, receiver) = std::sync::mpsc::sync_channel(10);

        let thread_join_handle = std::thread::spawn(move || {
            for task in receiver {
                match task {
                    Task::Stop => break,
                    Task::Future(future, sender) => {
                        PollingFutureRuntime::run_future(future, sender)
                    }
                    Task::Resume(_) => panic!("Not supported yet :-("),
                }
            }
        });

        PollingFutureRuntime {
            sender,
            thread_join_handle,
        }
    }

    fn run(&self, future: impl Future<Output = ()> + Unpin + Send + 'static) -> Handle {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1);
        let task = Task::Future(Box::new(future), sender);
        self.sender.send(task);

        Handle::new(receiver)
    }

    fn run_future(mut future: impl Future<Output = ()> + Unpin, sender: SyncSender<()>) {
        let waker = task::noop_waker_ref();
        let mut context = Context::from_waker(&waker);
        while let Poll::Pending = Pin::new(&mut future).poll(&mut context) {
            std::thread::sleep(Duration::from_secs(1));
        }

        sender.send(()).unwrap();
    }
}

fn do_something_async() -> impl Future<Output = ()> + Unpin {
    async {
        println!("do something");
        other_thing().await;
        println!("do something done");
    }
    .boxed()
}

async fn other_thing() {
    println!("other thing");
    sleep(Duration::from_secs(10)).await;
    println!("other thing is done");
}

fn sleep(duration: Duration) -> impl Future<Output = ()> + Unpin {
    let wake_up = Instant::now().add(duration);
    SleepFuture::new(wake_up)
}

struct SleepFuture {
    wake_up: Instant,
}

lazy_static! {
    static ref timer_service: Mutex<TimerService> = Mutex::new(TimerService::new());
}

struct TimerService {
    sender: SyncSender<Timer>,
    processor: JoinHandle<()>,
}

impl TimerService {
    fn new() -> TimerService {
        let (sender, receiver) = std::sync::mpsc::sync_channel(10);

        let mut timer_processor = TimerProcessor::new(receiver);

        let processor = std::thread::spawn(move || {
            timer_processor.run();
        });

        TimerService { sender, processor }
    }

    fn register(&self, wake_up: Instant, waker: Waker) {
        self.sender.send(Timer::new(wake_up, waker));
    }
}

struct TimerProcessor {
    receiver: Receiver<Timer>,
    timers: BinaryHeap<Timer>,
}

impl TimerProcessor {
    fn new(receiver: Receiver<Timer>) -> TimerProcessor {
        TimerProcessor {
            receiver,
            timers: BinaryHeap::new(),
        }
    }

    fn run(&mut self) {
        loop {
            let now = Instant::now();

            loop {
                match self.receiver.recv_timeout(Duration::from_millis(100)) {
                    Ok(timer) => {
                        if timer.wake_up <= now {
                            timer.waker.wake();
                        } else {
                            self.timers.push(timer);
                        }
                    }
                    Err(_) => break,
                }
            }

            while !self.timers.is_empty() {
                if self.timers.peek().unwrap().wake_up <= now {
                    let timer = self.timers.pop().unwrap();
                    timer.waker.wake();
                } else {
                    break;
                }
            }

            std::thread::sleep(Duration::from_millis(100));
        }
    }
}

struct Timer {
    wake_up: Instant,
    waker: Waker,
}

impl Timer {
    fn new(wake_up: Instant, waker: Waker) -> Timer {
        Timer { wake_up, waker }
    }
}

impl PartialEq<Self> for Timer {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl Eq for Timer {}

impl PartialOrd<Self> for Timer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.wake_up.partial_cmp(&other.wake_up)
    }
}

impl Ord for Timer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.wake_up.cmp(&other.wake_up)
    }
}

impl Future for SleepFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let now = Instant::now();

        if now < self.wake_up {
            let mut lock = timer_service.lock().unwrap();

            lock.register(self.wake_up, cx.waker().clone());
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

impl SleepFuture {
    fn new(wake_up: Instant) -> SleepFuture {
        SleepFuture { wake_up }
    }
}
