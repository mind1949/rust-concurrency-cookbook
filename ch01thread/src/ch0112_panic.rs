use std::{thread, time};

pub fn panic_example() {
    println!("Hello World");
    let handle = thread::spawn(|| {
        std::thread::sleep(time::Duration::from_millis(100));
        panic!("boom");
    });

    match handle.join() {
        Ok(r) => println!("All is welll: {:?}", r),
        Err(e) => println!("Got an error: {:?}", e),
    }
}

pub fn panic_caught_example() {
    println!("hello panic_caught_example");
    let handle = std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_millis(100));
        let result = std::panic::catch_unwind(|| {
            panic!("boom");
        });
        println!("panic caught, reuslt = {:?}", result);
    });

    match handle.join() {
        Ok(r) => println!("All is welll: {:?}", r),
        Err(e) => println!("Got an error: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panic_example() {
        panic_example();
    }

    #[test]
    fn test_panic_caught_example() {
        panic_caught_example();
    }
}
