/*
CHICO by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Declaring the
/// "modules" directory
/// as a module.
pub mod modules;

/// We re-export the module
/// for working with base-2 numbers.
pub use modules::binary::*;

/// We re-export the module
/// for working with base-16 numbers.
pub use modules::hexadecimal::*;