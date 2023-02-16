/// Declaring the
/// "modules" directory
/// as a module.
mod modules;

// Setting up tests.
#[cfg(test)]
pub use modules::tests::*;

/// Re-exporting the main
/// modules.

/// The "modules/utils.rs"
/// file.
pub use modules::utils::*;

/// The "modules/binary.rs"
/// file.
pub use modules::binary::*;