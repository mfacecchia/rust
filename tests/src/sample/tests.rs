use crate::sample::sample_functions::*;

// NOTE: That's another way of writing tests
// (separated from the business code in a separated module)
// (my preferred way of organizing tests but not the opinionated way)
#[test]
fn sample_add_should_return_four() {
    assert_eq!(add(2, 2), 4)
}