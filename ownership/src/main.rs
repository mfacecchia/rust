use std::io::{self, Write};

fn main() {
    // NOTE: Here, we initialize a new String Object and then pass it to the method `print_mesage_inline`,
    // which will take the ownership of the variable.
    // Once the function will complete its execution, the memory will be freed and not be accessible anymore.
    let simple_message = String::from("Hello, world!");
    print_message_inline(simple_message);
    // NOTE: If we try to print the variable's value now, it will throw an error,
    // because the variable's value will point to a nullish memory address

    // ```println!("{simple_message}");```

    // NOTE: In this case, instead, the method we declare will still take the String,
    // work with it, and then return it back again
    let new_message = String::from("Should execute with no problems");

    // Now, since we are assiging the returned value to a new variable, `new_message` will not be available again,
    // in order to ensure memory allocation/deallocation security (since when two variables share the same value,
    //                                                             deallocation can be a little difficult to do due to possible
    //                                                             double deallocation and consequent memory corruption)
    let new_message_owns_new_message = print_message_and_give_back_ownership(new_message);

    println!("{new_message_owns_new_message}");
    println!("In fact it works fine!");

    // NOTE: Now we are passing to the function the String reference, so basically the pointer to the value we are passing.
    // Since we are not passing the variable itself, the function can execute all types of operations
    // on that value without, in fact, having any ownership to the variable itself.
    // NOTE: Just like variables, also references are immutable by default.
    // In order for function to be able to update the value, the type will need to be marked as mutable,
    // just like in this example function
    // NOTE: There can be ONLY ONE MUTABLE POINTER AT A TIME to prevent race conditions,
    // and there cannot be ANY mutable reference if there is AT LEAST ONE immutable reference to the same value for the same reason
    let mut new_message_reference = String::from("This will be passed by reference");
    print_message_from_reference(&mut new_message_reference);
    println!("{new_message_reference}");
    println!("Reference passed successfully!");
}

fn print_message_inline(message: String) -> () {
    print!("{message}");
    io::stdout().flush().unwrap();
}

fn print_message_and_give_back_ownership(message: String) -> String {
    println!("{message}");
    return message;
}

fn print_message_from_reference(message: &mut String) -> () {
    println!("{message}");
    message.push_str(", updated string.");
}
