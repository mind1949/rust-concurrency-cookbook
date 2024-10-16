pub fn rayon_scope() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    rayon::scope(|s| {
        s.spawn(|_| {
            println!("Hello from first rayon scope thread");
            dbg!(&a);
        });
        s.spawn(|_| {
            println!("Hello from second rayon scope thread");
            x += a[0] + a[2];
        });
        println!("Hello from the main thread");
    });

    a.push(4);
    assert_eq!(x, a.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rayon_scope() {
        rayon_scope();
    }
}
