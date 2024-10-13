use std::{thread, time};
use thread_control;

pub fn control_thread() {
    let (flag, control) = thread_control::make_pair();
    let sleep_duration = time::Duration::from_millis(100);
    let handle = thread::spawn(move || {
        while flag.alive() {
            thread::sleep(sleep_duration);
            println!("I'm alive");
        }
    });

    thread::sleep(sleep_duration * 5);
    assert_eq!(control.is_done(), false);
    control.stop();
    handle.join().unwrap();

    assert_eq!(control.is_interrupted(), false);
    assert_eq!(control.is_done(), true);

    println!("This thread is stopped")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control_thread() {
        control_thread();
    }
}
