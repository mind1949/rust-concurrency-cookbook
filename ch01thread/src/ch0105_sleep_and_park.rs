use core::time;
use std::{thread, time::Duration};

pub fn start_thread_with_sleep() {
    for handle in (0..2)
        .map({
            |i| {
                thread::spawn(move || {
                    thread::sleep(Duration::from_millis(i * 1000));
                    println!("Hello from thread{} after sleep", i);
                })
            }
        })
        .collect::<Vec<_>>()
    {
        handle.join().unwrap();
    }
}

pub fn start_thread_with_yeild_now() {
    for handle in (0..2)
        .map(|i| {
            thread::spawn(move || {
                thread::yield_now();
                println!("Hello from thread{} after yield_now", i)
            })
        })
        .collect::<Vec<_>>()
    {
        handle.join().unwrap();
    }
}

pub fn thread_park2() {
    let handle = thread::spawn(|| {
        thread::sleep(time::Duration::from_millis(1000));
        thread::park();
        println!("Hello from a park thread in case of unpark first!");
    });

    handle.thread().unpark();
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_thread_with_sleep() {
        start_thread_with_sleep();
    }

    #[test]
    fn test_start_thread_with_yeild_now() {
        start_thread_with_yeild_now();
    }

    #[test]
    fn test_thread_park2() {
        thread_park2();
    }
}
