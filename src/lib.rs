// Add the documentation from the README.md file to the crate root documentation
#![doc = include_str!("../README.md")]

/// I needed something simple to put in the crate, just to see the generated
/// documentation.
pub fn hello_world() -> String {
    "Hello, world!".to_owned()
}
