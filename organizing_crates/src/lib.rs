// NOTE: The `lib.rs` file is also known as the `library crate`,
// meaning that it's the entry point for all other modules in the codebase.

// NOTE: This is a public module.
// Can be seen as a namespace with a collection of functions.
// All the functions and data structures in it can be private,
// and public (by defining it with the kewyword `pub`, just like we do with modules).
// All public modules, methods, and everything else can be imported by other modules
// using the `use` keyword and by specifying the module name and path to the required resource
// The private ones, instead, are accessible to the namespace itself.
pub mod developer;
pub mod simple_user;
mod geek_user;



// NOTE: Here, we are doing the so-called re-exporting,
// meaning that we are exporting something from a private module,
// from a place where that module is accessible, and exporting its
// public content (in this case just the method for printing out exhadecimal value).
// This is useful in cases where we want to hide the base module but make available to the world
// some parts of itself.
pub use crate::geek_user::print_hexadecimal_value;
