use std::{ops::Deref, time::SystemTime};


#[allow(dead_code)]
#[derive(Debug)]
struct Post {
    author: String,
    details: PostDetails,
    likes: u32
}

#[allow(dead_code)]
#[derive(Debug)]
struct PostDetails {
    content: String,
    uploaded_at: u128,
    uploaded_by: u32,
}

impl PostDetails {
    fn new(content: &str, uploaded_at: u128, uploaded_by: u32) -> PostDetails {
        PostDetails {
            content: content.to_owned(),
            uploaded_at,
            uploaded_by
        }
    }
}

impl Post {
    fn new(author: &str, details: PostDetails, likes: u32) -> Post {
        Post {
            author: author.to_owned(),
            details,
            likes
        }
    }
}

// NOTE: By default, Rust does not provide a way of dereferencing
// our custom structs, so we need to implement a method for doing so.
// It's possible to define such behavior by implementing the `Deref` trait.
impl Deref for Post {
    type Target = PostDetails;

    // NOTE: Here, we are telling the compiler to return a pointer to our `Details`
    // whenever it needs to deref a `Post` variable type
    fn deref(&self) -> &Self::Target {
        // NOTE: By adding a printout at the beginning of this method, we can
        // better understand when this method is actually called.
        println!("Dereferencing ongoing!");
        &self.details
    }
}

fn main() {
    let current_time_duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);

    let my_post_details = PostDetails::new("I am writing code in Rust!", current_time_duration.unwrap().as_millis(), 1);
    let my_new_post = Post::new("Feis._.", my_post_details, 2500);
    // NOTE: Here, even though we are passing a `Post` and the function required a `PostDetails`,
    // Rust compiler is able to  coerce the information by calling the `deref` implemented method
    // (just like it does by default with `String` and `&str`!)
    print_details(&my_new_post);
    print_post_data(&my_new_post);
}

fn print_details(post_details: &PostDetails) -> () {
    println!("{post_details:#?}");
}

fn print_post_data(post: &Post) -> () {
    println!("{post:#?}");
}
