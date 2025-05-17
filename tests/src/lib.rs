pub mod sample;

pub struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn new(width: i32, height: i32) -> Rectangle {
        // NOTE: While running tests, printouts will be ignored
        // unless the test fails. If we want to print out the string
        // whether the test fails or passes, we need to set the `--show-output` flag
        // while running `cargo test` (`cargo test -- --show-output`)
        println!("New!");
        if width == height {
            panic!("Rectangle must have different width and height, duh.")
        }
        Rectangle {
            width,
            height
        }
    }

    pub fn area(&self) -> i32 {
        (self.width * self.height) / 2
    }

    pub fn can_hold_smaller(&self, other: &Rectangle) -> bool {
        if self.width >= other.width
            && self.height >= other.height {
            return true;
        }
        false
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_should_return_four() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn test_should_panic() {
        panic!("Panics as expected.");
    }

    // NOTE: It's possible that the program panics for various reasons.
    // We use the `expected` param with a string as value to assure that the
    // panic error message matches our expected failure
    // (in this case, it will pass only if the `panic!()` message contains
    // "must have different width and height", and will fail if not)
    #[test]
    #[should_panic(expected = "must have different width and height")]
    fn test_rect_new_rectangle_should_panic() {
        Rectangle::new(2, 2);
    }

    #[test]
    fn test_rect_area_should_return_three() {
        let rect = Rectangle::new(2, 3);
        assert_eq!(rect.area(), 3);
    }

    #[test]
    fn test_rect_can_hold_smaller_should_return_true() {
        let rect1 = Rectangle::new(3, 4);
        let rect2 = Rectangle::new(2, 4);
        assert!(rect1.can_hold_smaller(&rect2));
    }

    // NOTE: It's possible to ignore the execution of certain tests
    // by simply setting the `ignore` attribute to the text function
    // This way, the test will not be executed unless specified
    // while running the `cargo test` with the `-include-ignored` argument
    // (`cargo test -- --include-ignored`).
    // It's also possible to only execute the ignored test fuctions
    // by passing the `--ignored` arg to `cargo test` (`cargo test -- --ignored`)
    #[test]
    #[ignore]
    fn test_rect_can_hold_smaller_should_return_false() {
        let rect1 = Rectangle::new(3, 4);
        let rect2 = Rectangle::new(2, 6);
        // NOTE: Custom failure message
        assert!(!rect1.can_hold_smaller(&rect2), "2nd rectangle is larger.");
    }

    // NOTE: It's also possible to make the test fail or pass
    // by using the `Result<>` enum. If the returned value is `Ok()`,
    // then it will pass, otherwise if `Err()` returned, it will fail
    // and return the defined message
    #[test]
    fn test_rect_can_hold_smaller_should_return_true_variant() -> Result<(), String> {
        let rect1 = Rectangle::new(3, 4);
        let rect2 = Rectangle::new(2, 4);

        if rect1.can_hold_smaller(&rect2){
            return Ok(());
        }
        Err("2nd rectangle is too large.".to_owned())
    }
}
