use std::sync::mpsc;
use std::thread;

pub struct Pool {
    workers: Vec<Worker>,
}

impl Pool {
    pub fn new(num_threads: u32) -> Pool {
        let mut workers = Vec::with_capacity(num_threads as usize);

        for id in 0..num_threads {
            workers.push(Worker::new(id));
        }

        Pool { workers }
    }

    pub fn run_parallel(&self) {
        for w in self.workers.iter() {
            w.sender.send(Message::NewJob(Job {}));
        }

        self.wait_all();
    }

    fn wait_all(&self) {
        for w in self.workers.iter() {
            w.receiver.recv().unwrap();
        }
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        for w in self.workers.iter() {
            w.sender.send(Message::Terminate);
        }

        for w in self.workers.iter_mut() {
            if let Some(thread) = w.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Job {}

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: u32,
    sender: mpsc::Sender<Message>,
    receiver: mpsc::Receiver<()>,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: u32) -> Worker {
        let (sender, worker_receiver) = mpsc::channel();
        let (worker_sender, receiver) = mpsc::channel();

        let thread = thread::spawn(move || loop {
            match worker_receiver.recv().unwrap() {
                Message::NewJob(job) => {
                    println! {"Stuff {}", id};
                    worker_sender.send(());
                }
                Message::Terminate => break,
            }
        });

        Worker {
            id,
            sender,
            receiver,
            thread: Some(thread),
        }
    }
}
