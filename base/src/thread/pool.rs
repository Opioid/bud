use std::thread;
use std::sync::mpsc;

//use std::time;


pub struct Pool {
    workers: Vec<Worker>,
 //   sender: mpsc::Sender<Job>,
}

impl Pool {
    pub fn new(num_threads: u32) -> Pool {
        
        
        let mut workers = Vec::with_capacity(num_threads as usize);

        for id in 0..num_threads {
            workers.push(Worker::new(id));
        }

        Pool {
            workers,
        }
    }

    pub fn run_parallel(&self) {
        for w in self.workers.iter() {
            w.sender.send(Job{});
        }
    }
}

struct Job;

struct Worker {
    id: u32,
    sender: mpsc::Sender<Job>,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: u32) -> Worker {
        let (sender, receiver) = mpsc::channel();
        
        let thread = thread::spawn(move || {
            receiver.recv().unwrap();
            println!{"Stuff"};
        });

        Worker {
            id,
            sender,
            thread,
        }
    }
}
