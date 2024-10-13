use std::{cell::RefCell, thread};

pub fn start_thread_with_threadlocal() {
    thread_local!(static COUNTER: RefCell<u8> = RefCell::new(1));

    COUNTER.with(|c| {
        *c.borrow_mut() = 2;
    });

    for handle in (0..2)
        .map(|i| {
            thread::spawn(move || {
                COUNTER.with(|c| {
                    *c.borrow_mut() = 3 + i;
                });

                COUNTER.with(|c| {
                    println!("Hello from thread{}, c={}", i, *c.borrow());
                })
            })
        })
        .collect::<Vec<_>>()
    {
        handle.join().unwrap();
    }

    COUNTER.with(|c| {
        println!("Hello from main thread, c={}", *c.borrow());
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_thread_with_threadlocal() {
        start_thread_with_threadlocal();
    }
}
