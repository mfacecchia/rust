fn main() {
    println!("Hello, world!");
    let mut phrase = String::from("Hello, world!");
    let truncated = get_first_word(&phrase);

    // NOTE: Since we passed to the function a `String` pointer,
    // the resulting `%str` will point to a section of the passed Object (stored in the Heap),
    // so, when we try to clear the String, it throws an error because the function returned value
    // is still using an immutable pointer to the base String.

    // phrase.clear();

    println!("{truncated}");

    // By passing a &str pointer, we are actually using data from the stack,
    // so we don't have to worry about any immutable pointer being borrowed.
    let phrase_stack = "Truncated hello, world!";
    let truncated = get_first_word(&phrase_stack);
    println!("{truncated}");
    // Why's that?
    // Because by design, `&str` are actually String slices (therefore immutable),
    // so it's basically impossible to have issues like the one illustrated before!
    // Since they're immutable, it's not possible by any mean to change their content.
}

fn get_first_word(phrase: &str) -> &str {
    for (i, &byte) in phrase.as_bytes().iter().enumerate() {
        if byte == b' ' {
            return &phrase[0..i];
        }
    }
    return &phrase[..];
}
