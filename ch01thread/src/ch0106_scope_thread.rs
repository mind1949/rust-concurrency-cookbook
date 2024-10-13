use std::thread;

pub fn start_scoped_threads() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    thread::scope(|s| {
        s.spawn(|| {
            println!("Hello from first scoped trhead");
            dbg!(&a);
        });
        s.spawn(|| {
            println!("Hello from second scoped thread");
            x += a[0] + a[2];
        });
    });

    a.push(4);
    assert_eq!(x, a.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_scoped_threads() {
        start_scoped_threads();
    }
}
