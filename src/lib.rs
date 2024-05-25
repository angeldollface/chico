/*
CHICO by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// Declaring the
/// "modules" directory
/// as a module.
pub mod modules;

/// We re-export the module
/// that holds Chico's CLI.
pub use modules::cli::*;

/// We re-export the module
/// with some Chico-specific
/// utility functions.
pub use modules::utils::*;

/// We re-export the module
/// for handling errors
/// for this crate.
pub use modules::error::*;

/// We re-export the module
/// for working with base-2 numbers.
pub use modules::binary::*;

/// We re-export the module
/// for working with base-16 numbers.
pub use modules::hexadecimal::*;