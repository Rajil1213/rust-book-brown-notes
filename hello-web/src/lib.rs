use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    job_sender: Option<Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new [`ThreadPool`].
    ///
    /// `num_threads` is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// If `num_threads` is zero.
    pub fn new(num_threads: usize) -> ThreadPool {
        assert!(num_threads != 0);

        let (job_queue_tx, job_queue_rx) = mpsc::channel::<Job>();

        let mut workers = Vec::with_capacity(num_threads);

        let receiver = Arc::new(Mutex::new(job_queue_rx));
        for id in 0..num_threads {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            job_sender: Some(job_queue_tx),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.job_sender
            .as_ref()
            .expect("channel closed")
            .send(job)
            .unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.job_sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // assign separately to drop and release `MutexGuard` lock
            let message = receiver.lock().expect("mutex poisoned").recv();

            if let Ok(job) = message {
                println!("Worker {id} got a job; executing...");
                job();
            } else {
                println!("Shutting down thread for worker: {id}");
                break;
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
