use std::io::{Read, Seek};

use error_handling::files;

fn main() {
    let mut file = files::open_file_options("random_file.txt", true);
    files::write(&mut file, "Hello, world!\n");
    file.rewind().unwrap();
    let mut buf = String::new();
    let bytes_read = file.read_to_string(&mut buf).unwrap_or_else(| err | {
        panic!("Could not read file. {err:#?}");
    });
    println!("File opened/created");
    println!("File content:");
    println!("{buf}");
    println!("Bytes read:");
    println!("{bytes_read}");
    println!("Removing file");
    files::remove("random_file.txt");
    println!("Done.")
}

fn last_char_of_first_line(phrase: &str) -> Option<char> {
    // NOTE: It's also possible to use the `?` for `Option`s
    // In this case, if a value is returned, the function continues, otherwise
    // early returns with `None`
    phrase.lines().next()?.chars().last()
}
