use std::{fs::File, io::{Error, ErrorKind, Write}};
use std::fs;

pub fn open_file(file_name: &str) -> File {
    // NOTE: Operations that may result in an error will return a `Result<T, E>`
    // type which will then be handled using either the `match` operator, or the `if let`
    let file = File::open(file_name);
    return match file {
        Ok(file) => {
            println!("File found, opening...");
            file
        },
        Err(err) => {
            // NOTE: To differentiate between all error types,
            // it's possible to `match` the error `kind()` from the `std::io::errorKind`
            // enum`match` the error `kind()` from the `std::io::errorKind`
            // various enums
            match err.kind() {
                ErrorKind::NotFound => {
                    println!("File not found, creating...");
                    match File::create(file_name) {
                        Ok(file) => {
                            println!("Done.");
                            file
                        },
                        // NOTE: If it's an error that the program cannot recover from,
                        // then it's possible to make the program `panic` and crash immediately
                        // (will automatically panic if the error is not handled as well)
                        Err(err) => panic!("Cannot create file! {err:#?}")
                    }
                }
                _ => panic!("Unknown error while opening file {err:#?}")
            }
        }
    }
}

pub fn open_file_not_handle(file_name: &str) -> Result<File, Error> {
    // NOTE: It's posible to also not directly handle the error and
    // make it handle on the calling function just by using the `?` operator
    // If the returned value from `Result` is `Ok`, then the function will continue with execution,
    // otherwise, return a `Result` with `Error`. This way we will be able to
    // keep the program cleaner.
    let file = File::open(file_name)?;
    // NOTE: If the function ends up successfully, then we can
    // return with `Ok` and the value as content
    Ok(file)
}

pub fn open_file_options(file_name: &str, create: bool) -> File {
    let file = File::options()
        .write(true)
        .truncate(true)
        .read(true)
        .create(create)
        // NOTE: If we want to get the error from `Result` (`Ok`),
        // and do something in case of `Error`, then it's possible to
        // `unwrap_or_else` with the variable that the `Error` will be assigned to
        .open(file_name).unwrap_or_else(| err | {
            panic!("Unknown error while processing file. {err:#?}");
        });
    file
}

pub fn write(file: &mut File, buf: &str) -> () {
    file.write_all(buf.as_bytes()).unwrap();
}

pub fn remove(file_name: &str) -> () {
    // NOTE: If we don't want to handle the `Error` and panic
    // with a custom message, it's possible to `expect` the function outcome
    // in order to improve debugging
    fs::remove_file(file_name).expect("Could not remove file");
}