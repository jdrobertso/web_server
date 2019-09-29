use std::thread;
use std::sync::mpsc;

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

struct Job;

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);
    let mut workers = Vec::with_capacity(size);
    let (sender, receiver) = mpsc::channel();

    for id in 0..size {
      workers.push(Worker::new(id));
    }

    ThreadPool {
      workers,
      sender,
    }
  }

  pub fn execute<F>(&self, f: F)
    where
      F: FnOnce() + Send + 'static
  {
    
  }
}

impl Worker {
  fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
    let thread = thread::spawn(|| {
      receiver;
    });

    Worker {
      id,
      thread,
    }
  }
}
