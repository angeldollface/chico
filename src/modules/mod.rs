/*
CHICO by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// We export the module
/// that holds Chico's CLI.
pub mod cli;

/// We declare a module
/// for testing our code.
#[cfg(test)]
pub mod tests;

/// We export the module
/// for working with 
/// base-8 numbers.
pub mod octal;

/// We export the module
/// for handling errors
/// for this crate.
pub mod error;

/// We export the module
/// for working with 
/// base-2 numbers.
pub mod binary;

/// We export the module
/// for working with 
/// base-16 numbers.
pub mod hexadecimal;