pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

// NOTE: Generally, you would write tests for each module in the same crate
// under a specific `test` module, mainly because you have access to private
// functions as well (which would normally be inaccessible from other external modules)
// (opinionated solution)
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_add_should_return_four() {
        assert_eq!(add(2, 2), 4)
    }
}
