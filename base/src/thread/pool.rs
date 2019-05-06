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

        self.wait_all();
    }

    fn wait_all(&self) {
        for w in self.workers.iter() {
            w.receiver.recv().unwrap();
        }
    }
}

struct Job;

struct Worker {
    id: u32,
    sender: mpsc::Sender<Job>,
    receiver: mpsc::Receiver<()>,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: u32) -> Worker {
        let (sender, worker_receiver) = mpsc::channel();
        let (worker_sender, receiver) = mpsc::channel();
        
        let thread = thread::spawn(move || {
            loop {
                worker_receiver.recv().unwrap();
                println!{"Stuff {}", id};

                worker_sender.send(());
            }
        });

        Worker {
            id,
            sender,
            receiver,
            thread,
        }
    }
}
