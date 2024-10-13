use std::thread;

pub fn start_one_thread_by_builder() {
    let builder = thread::Builder::new()
        .name("foo".to_string())
        .stack_size(32 * 1024);

    let handle = builder
        .spawn(|| {
            println!("Hello from a thread builder");
        })
        .unwrap();

    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_one_thread_by_builder() {
        start_one_thread_by_builder();
    }
}
