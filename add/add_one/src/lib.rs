pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_also_works() {
        let num = 10;
        assert_eq!(add_one(num), 11);
    }
}

// To run tests in this specific crate, `cargo test -p <crate_name>`
