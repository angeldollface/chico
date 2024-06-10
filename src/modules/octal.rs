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
pub fn is_octal(number: &str) -> bool {
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
pub fn octal_to_dec(number: &str) -> Result<u32, ChicoError> {
    let mut result: u32 = 0;
    let mut num_chars: Vec<char> = number.chars().collect::<Vec<char>>();
    num_chars.reverse();
    for (index, character) in num_chars.into_iter().enumerate(){
        let conv: u32 = match character.to_string().parse(){
            Ok(conv) => conv,
            Err(e) => return Err::<u32,ChicoError>(ChicoError::new(&e.to_string()))
        };
        let idx: u32 = index as u32;
        let to_add = (8_u32.pow(idx)) * conv;
        result += to_add;
    }
    Ok(result)
}

/// Attempts to convert a base-10 number as a 
/// string into a base-8 number.
pub fn dec_to_octal(number: &str) -> Result<String, ChicoError>{
    let mut octal_num_chars: Vec<String> = Vec::new();
    let integer: u32 = match number.to_string().parse::<u32>(){
        Ok(integer) => integer,
        Err(e) => return Err::<String, ChicoError>(ChicoError::new(&e.to_string()))
    };
    let mut div_res: u32 = integer / 8;
    while div_res != 0 {
        let octal_char: u32 = div_res % 8;
        println!("{}", div_res);
        octal_num_chars.push(octal_char.to_string());
        div_res = div_res / 8;
    }
    Ok(octal_num_chars.join(""))
}
