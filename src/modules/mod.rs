/*
CHICO by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// We declare a module
/// for testing our code.
#[cfg(test)]
pub mod tests;

/// We export the module
/// with some Chico-specific
/// utility functions.
pub mod utils;

/// We export the module
/// for working with base-2 numbers.
pub mod binary;

/// We export the module
/// for working with base-16 numbers.
pub mod hexadecimal;