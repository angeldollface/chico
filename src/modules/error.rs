/*
CHICO by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the standard
/// "Result" enum.
use std::fmt::Result;

/// Importing the standard
/// "Display" trait.
use std::fmt::Display;

/// Importing the standard
/// "Error" trait.
use std::error::Error;

/// Importing the standard
/// "Formatter" trait.
use std::fmt::Formatter;

/// A data structure for
/// storing and handling errors.
#[derive(Clone,Eq,PartialEq, Debug)]
pub struct ChicoError {
    pub details: String
}

/// Implement generic methods.
impl ChicoError {

    /// Implements a generic method to create
    /// a new instance of this data structure.
    pub fn new(details: &str) -> ChicoError {
        return ChicoError {
            details: details.to_owned()
        };
    }

    /// Implements a generic method to return
    /// a string representation of this 
    /// data structure.
    pub fn to_string(self) -> String {
        return self.details.to_string();
    }
}

/// Implements the "Error" trait.
impl Error for ChicoError {
    fn description(&self) -> &str {
        &self.details
    }
}

/// Implements the "Display" trait.
impl Display for ChicoError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return write!(f,"{}",self.details);
    }
}