use add_one;

fn main() {
    let num = 10;
    println!("Hello world! {num} plus one is {}", add_one::add_one(num));
}

// To run the binary of a crate from the root of a cargo workspace,
// Call `cargo run -p <crate_name>`.
