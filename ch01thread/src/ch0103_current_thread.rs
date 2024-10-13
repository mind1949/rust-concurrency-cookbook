use std::thread::{self};
use std::time;

pub fn current_thread() {
    let current_thread = thread::current();
    println!(
        "current thread: {:?}:{:?}",
        current_thread.id(),
        current_thread.name()
    );

    thread::Builder::new()
        .name("foo".to_string())
        .stack_size(32 * 1024)
        .spawn(|| {
            let current = thread::current();
            println!("current_thread: {:?}:{:?}", current.id(), current.name())
        })
        .unwrap()
        .join()
        .unwrap();
}

pub fn current_thread_park() {
    let parked_thread = thread::Builder::new()
        .spawn(|| {
            println!("Parking thread");
            thread::park();
            println!("Thread unparked");
        })
        .unwrap();

    thread::sleep(time::Duration::from_millis(10));

    println!("unpark the thread");
    parked_thread.thread().unpark();

    parked_thread.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_thread() {
        current_thread();
    }

    #[test]
    fn test_current_thread_park() {
        current_thread_park();
    }
}
