use std::fmt::Debug;

use generics::sample::{SampleStruct2, Summarizable2};

// NOTE: Traits are something like what we call
// "interfaces" in other programming languages
// We can create some abstract methods, default methods,
// and bind them to our structs
trait Summarizable {
    fn summarize(&self) -> ();

    fn print_summary(&self) -> () {
        println!("This method is supposed to print the summary but I don't know what to do print so here's some random words.");
    }
}

// NOTE: Defining a generic type that will be used
// to use the same struct for multiple data types
struct SampleStruct<T> {
    value: T
}

#[derive(Debug)]
struct SampleStructNonGeneric {
    value: u32
}

impl SampleStructNonGeneric {
    fn new(value: u32) -> SampleStructNonGeneric {
        SampleStructNonGeneric {
            value
        }
    }
}

// NOTE: In order to use a generic in methods implementations we need to
// define the same type in the `impl` itself, and in the struct
impl<T> SampleStruct<T> {
    fn new(value: T) -> SampleStruct<T> {
        SampleStruct{
            value
        }
    }

    fn value(&self) -> &T {
        &self.value
    }
}

// NOTE: We can also specify a set of methods that will be available
// for a concrete type only (in this case the method `multiply` only for
// structs with the `i32` concrete type)
impl SampleStruct<i32> {
    fn multiply(&self) -> i32 {
        self.value * 2
    }
}

// NOTE: Implementing all the trait's abstract methods in the
// `SampleStruct` with concrete type `String`
impl Summarizable for SampleStruct<String> {
    fn summarize(&self) -> () {
        println!("New message!");
        println!("Content: {}", self.value);
    }
}

// NOTE: It's possible to implement a trait for all structs
// that implement a specific trait (or group of traits) as well
// like in this example
impl<T: Debug> Summarizable for T {
    fn summarize(&self) -> () {
        println!("Implementing this method for all structs that implement the `Debug` trait.");
    }
}

// NOTE: This is not allowed because
// only traits defined in the current scope can be implemented in local AND external structs and enums
// and only structs defined in the local scope can implement traits defined in local AND external crates
// For example `String` cannot implement the `Display` trait
// because they are both defined in an external crate
// This is done to ensure security
// impl Summarizable2 for SampleStruct2 {}

fn random_summary(value: &impl Summarizable) -> () {
    println!("Calling implmented trait from function.");
    value.summarize();
}

// NOTE: Defining a function that requires multiple traits to be implemented
// In this case we are using the `where` clause to simplify the code readability
// and assign a type to each generic
fn random_summary_multiple_traits<T>(value: &T) -> ()
where T: Summarizable + Debug {
    println!("Calling implemented method from multiple traits function.");
    value.summarize();
}

// NOTE: We can also set the generic to be something like the following
fn random_summary_multiple_traits_alternative(value: &(impl Summarizable + Debug)) -> () {
    println!("Calling implemented method from multiple traits function.");
    value.summarize();
}

fn build_summarizable() -> impl Summarizable2 + Debug {
    SampleStruct2::new(5)
}

fn main() {
    let ss: SampleStruct<String> = SampleStruct::new(String::from("Remember to wash the dishes!"));

    ss.value();

    let int_ss: SampleStruct<i32> = SampleStruct::new(4);
    int_ss.multiply();

    ss.summarize();
    ss.print_summary();

    random_summary(&ss);

    let ssng = SampleStructNonGeneric::new(40);
    ssng.summarize();
}
