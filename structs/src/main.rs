use rand::{Rng, rng};
use uuid::Uuid;

// NOTE: Adding the `Debug` trait to the struct
// to enable struct data printout (ONLY USE IN DEVELOPMENT)
// For production you may want to implement `Display`
#[derive(Debug)]
struct User {
    user_id: Uuid,
    username: String,
    email: String,
    age: u32,
}

// NOTE: Here, we are adding some methods to our struct.
// Each method takes a `&self` parameter, referring to the variable calling it.
// As seen in previous exercises, it can take ownership of the variable, or it can take a pointer (using the `&`)
// (immutable by default, of course)
impl User {
    /// Formats email field to be uppercase, keeping consistent format
    fn format_email(&mut self) -> () {
        self.email = self.email.to_uppercase();
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }

    /// Builds a new User with the provided data and a uniquely generated user_id
    /// ## Params:
    ///     username [&str]: the username
    ///     email [&str]: the user email
    ///     age [u32]: the user age
    /// ## Returns:
    ///     User: a new User
    fn build_user(username: &str, email: &str, age: u32) -> User {
        return User {
            user_id: Uuid::new_v4(),
            email: String::from(email),
            username: String::from(username),
            age
        };
    }
}

// NOTE: Defining a Tuple Struct (a tuple with pre-defined items type definition for improved tuples reusability)
struct Point(f32, f32, f32);

fn main() {
    let mut user_1 = build_user("random@provider.com", "randomUser1", 18);

    // NOTE: Here, rust recognises we are calling a struct method requiring
    // a mutable pointer, and so automatically passes the pointer of the calling struct
    user_1.format_email();

    // Another approach to this is the following,
    // anyways, this is less readable so not really suggested
    // ```(&mut user_1).format_email();```

    user_1.is_adult();

    let user_2 = User {
        user_id: Uuid::new_v4(),
        email: user_1.email.clone(),
        username: user_1.username.clone(),
        ..user_1
    };

    println!(
        "That's the full user information:
    ID => {}
    Email => {}
    Username => {}
    Age => {}",
        user_1.user_id, user_1.email, user_1.username, user_1.age
    );

    println!(
        "That's the full 2nd user information:
    ID => {}
    Email => {}
    Username => {}
    Age => {}",
        user_2.user_id, user_2.email, user_2.username, user_2.age
    );

    println!("[DEBUG] user_1 = {user_1:#?}");

    let mut coords_1 = build_random_coordinates();

    println!(
        "Here's some random coordinates:
        X => {:.2}
        Y => {:.2}
        Z => {:.2}",
        coords_1.0, coords_1.1, coords_1.2
    );

    coords_1.0 = 2.00;

    // NOTE: It is possible to return struct values directly from structs themselves
    let built_user = User::build_user("builtUser1", "built@provider.com", 30);
    println!("{built_user:#?}");
}

fn build_user(email: &str, username: &str, age: u32) -> User {
    return User {
        user_id: Uuid::new_v4(),
        email: String::from(email),
        username: String::from(username),
        age,
    };
}

fn build_random_coordinates() -> Point {
    return Point(
        rng().random_range(0.00..300.00),
        rng().random_range(0.00..300.00),
        rng().random_range(0.00..300.00),
    );
}
