/*
CHICO by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// We import the data structure
/// for catching and handling errors.
use super::error::ChicoError;

/// This function checks whether
/// the supplied string is a
/// number of base-8 or not.
pub fn is_octal(number: &String) -> bool {
    let mut result: bool = true;
    let allowed: Vec<char> = "012345678".chars().collect();
    let num_chars: Vec<char> = number.chars().collect();
    for character in num_chars{
        if allowed.contains(&character){}
        else {
            result = false;
        }
    }
    result
}

/// Attempts to convert a base-8 number as a string
/// into a base-10 number and return it.
pub fn octal_to_dec(number: &String) -> Result<u32, ChicoError> {
    let mut result: u32 = 0;
    let mut num_chars: Vec<char> = number.chars().collect::<Vec<char>>();
    num_chars.reverse();
    for (index, character) in num_chars.into_iter().enumerate(){
        let num: u32 = match character.try_into(){
            Ok(num) => num,
            Err(e) => return Err::<u32,ChicoError>(ChicoError::new(&e.to_string()))
        };
        let idx: u32 = index as u32;
        let to_add = ((8 as u32).pow(idx)) * num;
        result = result + to_add;
    }
    return Ok(result);
}