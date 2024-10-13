use std::thread;

pub fn start_thread() {
    let handle = thread::spawn(|| println!("Hello from a thread!"));

    handle.join().unwrap()
}

pub fn start_two_threads() {
    let handle1 = thread::spawn(|| {
        println!("Hello from a thread1");
    });

    let handle2 = thread::spawn(|| {
        println!("Hello from a thread2");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

pub fn start_n_threads() {
    const N: isize = 10;

    let handles: Vec<_> = (0..N)
        .map(|i| {
            thread::spawn(move || {
                println!("Hello from thread{}", i);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_thread() {
        start_thread()
    }

    #[test]
    fn test_start_two_threads() {
        start_two_threads();
    }

    #[test]
    fn test_start_n_threads() {
        start_n_threads();
    }
}
