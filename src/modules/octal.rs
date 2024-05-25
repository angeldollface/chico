/*
CHICO by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// We import a function
/// to raise one number to another
/// from the "utils" module.
use super::utils::raise_to;

/// We import the data structure
/// for catching and handling errors.
use super::error::ChicoError;

/// We import a function
/// to split a string by
/// separator into a vector of
/// strings from the "utils" module.
use super::utils::clean_split;

/// We import a function
/// to reverse the order
/// of elements in a vector
/// from the "utils" module.
use super::utils::reverse_vec;

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
    if is_octal(number){
        let octal_chars: Vec<String> = clean_split(number, &String::from(""));
        let reversed: Vec<String> = reverse_vec(&octal_chars);
        for (index, elem) in reversed.iter().enumerate() {
            if elem == &String::from("1") {
                let base: u32 = match TryInto::<u32>::try_into(8){
                    Ok(base) => base,
                    Err(e: TryFromIntError) => return Err::<u32, ChicoError>(ChicoError::new(&e.to_string()))
                };
                let power: u32 = match index.try_into(){
                    Ok(power) => power,
                    Err(e) => return Err::<u32, ChicoError>(ChicoError::new(&e.to_string()))
                };
                let to_add: u32 = raise_to(
                    &base, &power
                );
                result += to_add;
            }
            else {}
        }
    }
    else {
        let e: String = format!("The string \"{}\" is not a binary number.", number);
        return Err::<u32, ChicoError>(ChicoError::new(&e.to_string()));
    }
    return Ok(result);
}