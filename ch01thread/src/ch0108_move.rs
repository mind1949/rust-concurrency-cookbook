use std::sync::Arc;
use std::thread;

pub fn start_one_thread_with_move() {
    let x = 10;
    (0..2)
        .map(|i| {
            thread::spawn(move || {
                println!("Hello from a thread{} with a move, x: {}", i, x);
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|handle| {
            handle.join().unwrap();
        })
}

pub fn start_one_thread_with_move2() {
    let x = vec![1, 2, 3];
    let x = Arc::new(x);
    (0..2)
        .map(|i| {
            let x = x.clone();
            thread::spawn(move || {
                println!("Hello from a thread{} with a move, x: {:?}", i, x);
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|handle| {
            handle.join().unwrap();
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_one_thread_with_move() {
        start_one_thread_with_move();
    }

    #[test]
    fn test_start_one_thread_with_move2() {
        start_one_thread_with_move2();
    }
}
