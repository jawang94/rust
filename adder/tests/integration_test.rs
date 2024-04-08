use adder;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

// no need to call #[cfg(test)], Cargo treats test/ direc specially and compiles files in here only when `cargo test` is run
