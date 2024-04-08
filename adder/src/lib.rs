pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn make_me_panic() {
    println!("printing before aiya panic");
    panic!("aiya");
}

fn using_result() -> Result<(), String> {
    if add(3, 3) == 7 {
        Ok(())
    } else {
        Err(String::from("wabi sabi mudda trucka"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic(expected = "ai")]
    fn another() {
        make_me_panic();
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }
    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }
    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    fn smaller_cannot_hold() {
        let larger = Rectangle {
            width: 50,
            height: 50,
        };
        let smaller = Rectangle {
            width: 25,
            height: 25,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[ignore]
    fn test_using_result() {
        assert!(!using_result().is_err(), "kenshi yonezu");
    }
}

// cargo test, cargo test <match_test_name>, cargo test --show-output, cargo test -- --ignored, cargo test --test integration_test
