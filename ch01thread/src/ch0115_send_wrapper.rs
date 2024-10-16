use send_wrapper::SendWrapper;
use std::{ops::Deref, rc::Rc, sync::mpsc, thread};

pub fn send_wrapper() {
    let value = Rc::new(42);
    let wrapper_value = SendWrapper::new(value);

    let (sender, receiver) = mpsc::channel();

    let handle = thread::spawn(move || {
        sender.send(wrapper_value).unwrap();
    });
    handle.join().unwrap();

    let wrapper_value = receiver.recv().unwrap();

    let value = wrapper_value.deref();

    println!("received from main thread: {}", value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_wrapper() {
        send_wrapper();
    }
}
