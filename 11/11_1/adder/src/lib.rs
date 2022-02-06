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
    a * 3
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
}
