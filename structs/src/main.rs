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

// NOTE: Defining a Tuple Struct (a tuple with pre-defined items type definition for improved tuples reusability)
struct Point(f32, f32, f32);

fn main() {
    let mut user_1 = build_user("random@provider.com", "randomUser1", 18);

    let user_2 = User {
        user_id: Uuid::new_v4(),
        email: user_1.email.clone(),
        username: user_1.username.clone(),
        ..user_1
    };

    user_1.email = user_1.email.to_uppercase();

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
