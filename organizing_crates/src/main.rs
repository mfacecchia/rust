// NOTE: When we `use` a module in our `main.rs` file, we are referring
// to the module declared in the `lib.rs` file like in this case.
// If we decided to make the `developer` modle private, we woulnd't have been able
// to import it
// If we would've decided to make the `developer` module private, we woulnd't have been able
// to import it in our code
use organizing_crates::developer::conversions;

fn main() {
    let random_value: i32 = 123;
    conversions::print_hex(&random_value);
}