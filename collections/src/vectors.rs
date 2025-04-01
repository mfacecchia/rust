enum AllowedVectorTypes {
    Integer(i32),
    UnsignedInteger(i32),
    Text(String),
}

pub fn work_with_vectors() -> () {
    // NOTE: This is a vector.
    // We use it to store multiple items of the same type dynamically
    // (items in this collections can be added ad removed at runtime,
    // therefore, its size is dynamic and most suitable for all use cases
    // than a normal Array with static size)
    let mut vector: Vec<i32> = Vec::new();

    // Adding items on top of the vector
    vector.push(3);
    vector.push(5);

    // Removing the last item in the vector
    vector.pop();

    // Adding an entry at the specified index
    vector.insert(1, 9);
    vector.insert(1, 5);

    vector.remove(1);

    // NOTE: It's possible to obtain an entry from the vector in two ways
    // The first one consist in calling the `get` method from the vector
    // (safest way because it returns an Option) based on whether the
    // index passed is valid (therefore returns the item) or not (`Option::None`)
    let index = 1;
    let number_option= vector.get(index);
    if let Some(fetched_number) = number_option {
        println!("The number at the Vector index {index} is {fetched_number}.");
    } else {
        println!("Index out of vector bounds!");
    };

    vector.push(2);

    // NOTE: Otherwise this other method
    // Dangerous because could crash the application if invalid
    // NOTE: Since it uses a borrow checker, it disables any type of mutation
    // of the whole vector (since it's subject to re-allocation to another location
    // in case of resizing).
    let number = &vector[0];
    println!("{number}");

    vector.push(2);

    // If we'd try to use the `number` value again as an immutable pointer,
    // the compiler would throw an error
    // println!("{number}");

    println!("{vector:?}");

    println!("vector size {}", vector.len());

    // NOTE: That's a faster way of initializing vectors with some values straightaway
    // Here, we are using the `Vec` inizialization macro
    let vector_simplified = vec![String::from("Hello"), String::from("World"), String::from("!")];

    println!("Printing all vector's items one at a time.");
    for entry in &vector_simplified {
        println!("{entry}");
    }

    // NOTE: It's possible to have "multiple vector types" using a simple workaround
    // By setting the vector type as a custom enum, it's possible to
    // define for each vector item one of the types defined in the vector.
    // To ensure type security, tho, it's required to use either a `match` or an `if let`
    // as usual.
    let mut multi_type_vector: Vec<AllowedVectorTypes> = Vec::new();

    multi_type_vector.push(AllowedVectorTypes::Text(String::from("Hello, world!")));
    multi_type_vector.push(AllowedVectorTypes::UnsignedInteger(32));
    multi_type_vector.push(AllowedVectorTypes::Integer(-40));

    println!("Printing all values from a multi type vector");
    for item in &multi_type_vector {
        match item {
            AllowedVectorTypes::Text(msg)  => {
                println!("This is a message. It says: \"{msg}\"");
            }
            AllowedVectorTypes::Integer(number) => {
                println!("This is a signed integer. The number is {number}");
            }
            AllowedVectorTypes::UnsignedInteger(number) => {
                println!("This is an unsigned integer. The number is {number}");
            }
        }
    }

    let incr_by: i32 = 10;
    println!("Increasing all vector items by {incr_by}.");
    increase_value(&mut vector, 10);
    println!("Updated values");
    println!("{vector:?}");
}

fn increase_value(vec: &mut Vec<i32>, incr_by: i32) -> () {
    vec.iter_mut().for_each(| entry | {
        // NOTE: Explicit dereferencing here instead
        *entry += incr_by;
    });
}