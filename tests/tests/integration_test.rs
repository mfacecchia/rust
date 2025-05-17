use tests::sample::sample_functions;

// NOTE: Integration tests go in a separated package
// (generally called `tests`) placed outside of the source
// code

#[test]
fn integration_sample_add_should_return_four() {
    assert_eq!(sample_functions::add(2, 2), 4);
}
