#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn add_two(a: i32) -> i32 {
    a + 2
}

fn multi_two(a: i32) -> i32 {
    a * 2
}

fn greeting(name: &str) -> String {
    format!("Hallo {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 20,
        };

        let smaller = Rectangle {
            width: 3,
            height: 6,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle {
            width: 10,
            height: 20,
        };

        let smaller = Rectangle {
            width: 3,
            height: 6,
        };

        assert!(!smaller.can_hold(&larger))
    }

    // it is possible to assert_eq!(left, right) to check left == right and
    // assert_ne!(left, right) to check left != right
    #[test]
    fn is_add_two() {
        let res = add_two(2);
        assert_eq!(res, 4) // the order of res and 4 is not important
    }

    #[test]
    fn is_multi_two() {
        let res = multi_two(5);
        assert_eq!(10, res) // failed because we define the bug in multi_two function
    }

    #[test]
    fn greeting_contains_name() {
        let res = greeting("Tom");
        // all values after first param in assert macro and second parameters assert_eq and assert_ne are message
        assert!(
            res.contains("Schayan"),
            "Greeting did not contain name, value is {}",
            res
        );
    }
}
