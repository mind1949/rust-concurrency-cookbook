use go_spawn;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

pub fn go_thread() {
    let counter = Arc::new(AtomicI64::new(0));
    let counter_cloned = counter.clone();

    go_spawn::go!({
        (0..100).into_iter().for_each(|_| {
            counter_cloned.fetch_add(1, Ordering::SeqCst);
        });
    });
    go_spawn::join!().unwrap();

    assert_eq!(counter.load(Ordering::SeqCst), 100);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_thread() {
        go_thread();
    }
}
