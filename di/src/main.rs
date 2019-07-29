extern crate base;
extern crate core;

mod options;

use base::math::int2;
use core::image;
use options::Options;
use std::env;

fn main() {
    println!("Welcome to di");

    let args: Vec<String> = env::args().collect();

    let options = Options::new(&args);
/*
    let available_threads = thread::available_threads();

    let num_workers;

    if options.threads <= 0 {
        let num_threads = available_threads as i32 + options.threads;
        num_workers = num_threads.max(1) as u32;
    } else {
        num_workers = available_threads.min(options.threads.max(1) as u32);
    }

    println!("#Threads {}", num_workers);
*/
}
