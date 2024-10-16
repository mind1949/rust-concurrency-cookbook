//use affinity;

#[cfg(not(target_os = "macos"))]
pub fn use_affinity() {
    let cores = (0..affinity::get_core_numa())
        .step_by(2)
        .collect::<Vec<usize>>();
    println!("Binding thread to cores : {:?}", &cores);

    affinity::set_thread_affinity(&cores).unwrap();
    println!(
        "Current thread affinity : {:?}",
        affinity::get_thread_affinity().unwrap(),
    );
}
