use std::thread;
use std::{sync::mpsc, sync::Arc, sync::Mutex};

pub struct ThreadPool {
    // threads: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

// struct Job;

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        // ThreadPool
        // let threads = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();
        // type Job = Box<dyn FnOnce() + Send + 'static>;
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create some threads and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }

}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {

    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                // println!("Worker {id} got a job; executing.");
                job();
            }
        });
        Worker { id, thread }
    }
}

    // fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    //     let thread = thread::spawn(move || {
    //         while let Ok(job) = receiver.lock().unwrap().recv() {
    //             println!("Worker {id} got a job; executing.");
    //             job();
    //         }
    //     });

    //     Worker { id, thread }
    // }
    // fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    //     let thread = thread::spawn(|| {
    //         receiver;
    //     });

    //     Worker { id, thread }
    // }
// }

// impl Worker {
//     fn new(id: usize) -> Worker {
//         let thread = thread::spawn(|| {});

//         Worker { id, thread }
//     }
// }