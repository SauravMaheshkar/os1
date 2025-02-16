pub fn test_runner(tests: &[&dyn Fn()]) {
    println_serial!("Running {} tests", tests.len());

    for test_fn in tests {
        test_fn();
    }
    unsafe {
        crate::io::serial::exit(0);
    }
}
