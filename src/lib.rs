use std::thread;
use std::sync::{mpsc, Mutex, Arc};
struct Worker {
    id: u16,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: u16, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let msg = rx.lock().unwrap().recv();
            
            match msg {
                Ok(job) => {
                    println!("Worker-{id} started");
                    job();
                }
                Err(_) => {
                    println!("Worker-{id} shutting down");
                    break;
                }
            }
        });
        Worker{id, thread: Some(thread)}
    }
}
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sx: Option<mpsc::Sender<Job>>,
}


impl ThreadPool {
    pub fn new(size: u16) -> ThreadPool {
        assert!(size > 0);

        let (sx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(size.into());

        for i in 0..size {
            let current_worker = Worker::new(i, Arc::clone(&rx));
            workers.push(current_worker);
        }
        ThreadPool{workers, sx: Some(sx)}
    }
    pub fn execute<Fn>(&self, f: Fn) 
    where 
        Fn: FnOnce() + Send + 'static {
            let job = Box::new(f);
           self.sx.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sx.take());

        for worker in &mut self.workers {
            println!("Working stop {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}