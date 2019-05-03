extern crate num_cpus;

pub mod pool;

pub use self::pool::Pool;

pub fn available_threads() -> u32 {
    num_cpus::get() as u32
}
