use num_cpus;
use num_threads;
use std::thread;

fn get_concurrency_count() {
    let count = thread::available_parallelism().unwrap();
    println!("available parallelism {}", count);

    let count = num_cpus::get();
    println!("cpus count {}", count);

    if let Some(count) = num_threads::num_threads() {
        println!("threads count {}", count);
    } else {
        println!("threads count: not support")
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_concurrency_count() {
        get_concurrency_count();
    }
}
