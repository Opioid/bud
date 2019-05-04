use std::thread;
//use std::time;

struct Inner {
    id: u32,
}

impl Inner {
    fn program(&self) {
        // for _ in 0..10 {
        //     println!("I am thread {}", self.id);

        //     thread::sleep(time::Duration::from_millis(200));
        // }
    }
}

pub struct Pool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl Pool {
    pub fn new(num_threads: u32) -> Pool {
        let mut pp = Pool {
            threads: Vec::new(),
        };

        for i in 0..num_threads {
            let inner = Inner { id: i };
            pp.threads.push(thread::spawn(move || {
                inner.program();
            }));
        }

        pp
    }

    pub fn run_parallel() {}

    pub fn wait_all(self) {
        for t in self.threads {
            t.join();
        }
    }
}

// impl Drop for Pool {
//     fn drop(&mut self) {
//         self.wait_all();

//         println!("Pool::drop()");
//     }
// }
