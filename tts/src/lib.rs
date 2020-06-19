pub fn add_two(a: u32) -> u32 {
    a + 2
}

fn greet(name: &str) -> String {
    String::from("Hello")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn test_greet() {
        let res = greet("jj");
        assert!(
            !res.contains("jj"),
            "Greetings did not contain name: `{}`",
            res
        );
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 6,
            height: 5,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 6,
            height: 5,
        };

        assert!(!smaller.can_hold(&larger));
    }
    #[test]
    #[should_panic]
    fn width_cant_be_to_big() {
        Rectangle::new(200, 300);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        if width < 0 || width > 100 {
            panic!("Width should be smaller than 100, got {}", width);
        }
        Rectangle { width, height }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
