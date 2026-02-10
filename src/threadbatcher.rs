use std::ops::Deref;
use std::sync::{Mutex, Arc};
use std::sync;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: sync::mpsc::Sender<ThreadPoolMessage>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = sync::mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static {
        let mut job = Box::new(f);
        self.sender.send(ThreadPoolMessage::NewJob(job)).unwrap();
    }
}

impl  ThreadPool {
    pub fn join(&mut self) {
        for worker in &mut self.workers {
            self.sender.send(ThreadPoolMessage::Terminate).unwrap();
        }
        while self.workers.len() > 0 {
            self.workers.pop().unwrap().thread.join().unwrap();
        }
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<sync::mpsc::Receiver<ThreadPoolMessage>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    ThreadPoolMessage::NewJob(job) => {
                        job.call_box();
                        
                    },
                    ThreadPoolMessage::Terminate => {
                        break;
                    },
                }
            }
            receiver;
        });

        Worker {
            id,
            thread,
        }
    }
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(mut self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

enum ThreadPoolMessage {
    NewJob(Job),
    Terminate,
}