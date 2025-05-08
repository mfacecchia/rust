fn main() {
    let str1 = String::from("Hello, world!");
    let result;
    {
        let str2 = String::from("Hello, world 2!");
        // NOTE: `result` valid here
        result = longest(&str2, &str1);
        println!("{result}");
    }
    // NOTE: `str2` goes out of scope so `result` will be invalidated as well
    // (possible returned pointer to be `str2` which is a dangling reference)
    // println!("{result}");
}

// NOTE: Defining a generic lifetime. This serves as a lifetime relationship
// annotation to be sure that the compiler will not return
// a possible dangling reference.
// In this case we are telling the compiler that `x` will live as long as `y` is valid
// and viceversa. Without the generic definition, the compiler would not be
// able to validate the lifetime relationship between the parameters and the returned value
// (since the method can be called from different places
// and parameters have different lifetimes between calls)
// NOTE: In this case the returned value will be valid as long as `x` and `y` are BOTH valid
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
