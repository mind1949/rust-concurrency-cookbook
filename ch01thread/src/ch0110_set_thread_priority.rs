use std::convert::TryInto;
use std::thread;
use thread_priority::{ThreadBuilderExt, ThreadPriority};

pub fn start_thread_with_priority() {
    vec![
        ThreadPriority::Max,
        ThreadPriority::Min,
        ThreadPriority::Crossplatform(0.try_into().unwrap()),
    ]
    .into_iter()
    .enumerate()
    .map(|(i, priority)| {
        thread::Builder::new()
            .name(format!("{}", i))
            .spawn_with_priority(priority, move |result| {
                // assert!(set_current_thread_priority(priority).is_ok());
                println!("Hello from thread{}, set priority result: {:?}", i, result);
                assert!(result.is_ok());
            })
    })
    .collect::<Vec<_>>()
    .into_iter()
    .for_each(|handle| handle.unwrap().join().unwrap());
}
