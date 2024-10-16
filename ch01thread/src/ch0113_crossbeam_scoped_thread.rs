pub fn crossbeam_scope() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    crossbeam::thread::scope(|s| {
        s.spawn(|_| {
            println!("hell from the first crossbeam scoped thread");
        });
        s.spawn(|_| {
            println!("hello from the second crossbeam scoped thread");
            x += a[0] + a[2];
        });
        println!("hell from the main thread");
    })
    .unwrap();

    a.push(4);
    assert_eq!(x, a.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crossbeam_scope() {
        crossbeam_scope();
    }
}
