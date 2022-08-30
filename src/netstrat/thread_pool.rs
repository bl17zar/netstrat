use std::thread;

use crossbeam::channel::{unbounded, Receiver, Sender};
use tracing::{debug, trace, error};

struct Job {
    f: Box<dyn FnOnce() + Send + 'static>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Receiver<Job>) -> Self {
        debug!("initing worker with id: {id}");

        let thread = thread::spawn(move || loop {
            if let Ok(job) = receiver.recv() {
                trace!("worker {id} got a job; executing.");

                let f = job.f;
                f();
            }
        });

        Self { id, thread }
    }
}
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
}

// TODO: gracefull shutdown
impl ThreadPool {
    pub fn new(size: usize) -> Self {
        debug!("initing thread_pool");
        let (sender, receiver) = unbounded();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        Self { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Job { f: Box::new(f) };

        if let Err(err) = self.sender.send(job) {
            error!("failed to send job with err: {err}");
        };
    }
}