// NOTE: Importing the `io` library from the base `std` library to obtain input from user
use std::{cmp::Ordering, io::{self, Write}};
use rand::Rng;

fn main() {
    println!("---- Guessing game!! ----");
    println!("|   Guess the number!   |");
    println!("-------------------------\n");

    let secret_number = gen_rand();
    println!("The secret number is {secret_number}.");

    // NOTE: That's a `while (true)` expression sort of. BE CAREFUL!!
    loop {
        print!("Input a random number (type \"quit\" to quit the program): ");
        // NOTE: Flushing the output stream in order to immediately printout the message
        // (rust waits for a `\n` before printing out, therefore it gets kept in the stream waiting for such char).
        // We are `unwrap()`ping the string because we don't care of storing the returned value, so by unwrapping
        // we are "throwing out" such result (if not done, it will throw an `unhandled value` warning)
        io::stdout().flush().unwrap();

        // NOTE: Creating a new Object of the `String` class and calling it `guess`
        // We use the `let` keyword to define a variable, adding the `mut` keyword we make our variable 
        // subjectable to changes during its lifecycle (immutable by default)
        let mut guess = String::new();
            io::stdin()
                // NOTE: Assiging the input result to the `guess` variable (needs to be immutable tho)
                // The `&` means that we are passing the pointer of the variable
                // The value assigned to the variable is ALWAYS A STRING
                .read_line(&mut guess)
                // NOTE: Expecting possible errors. In case of error, will printout the string without crashing the program.
                // Can compile without handling but will throw a warning.
                .expect("Unexpected error while getting input.");

        match is_quit(&guess) {
            true => break,
            // NOTE: Empty parenthesis means "do nothing"
            false => ()
        };

        // Parsing the guess from string to a 32 bit integer value
        // Handling the error in a way that will not make the app crash (in case of error, it would throw an error)
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input!");
                continue;
            },
        };

        println!("You guessed: {guess}");

        // NOTE: Match expressions allow us to check for all types of patterns (a.k.a. "arms")
        // The first successful match quits the arms checking
        // (writing checks this way is comparable to wriring tons of `if` `if else` clauses but in a prettier manner)
        match guess.cmp(&secret_number) {
            // That's an arm!
            Ordering::Equal => {
                println!("You got it right!!");
                break;
            },
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too large!"),
        };
    }
}

// NOTE: Declaring a function returning an 32 bit integer value
fn gen_rand() -> i32 {
    let rand = rand::rng().random_range(1..=100);
    return rand;
}

/// Checks wheter the user wants to quit the game or not
/// ## Returns:
///     true: the user wants to quit
///     false: the user wants to play the game
fn is_quit(guess: &String) -> bool {
    // NOTE: Using the `as_str()` method from the `String` class in order to be able to compare two `&str`
    // (cannot, in fact, compare `String` and `&str`)
    return match guess.trim().to_uppercase().as_str() {
        "QUIT" => {
            println!("Thank you for playing!");
            return true;
        },
        // NOTE: Default case to do nothing (e.g. continue with code normal flow)
        // (required by rust because it needs to check all possible matches from the expression)
        _ => false,
    };
}