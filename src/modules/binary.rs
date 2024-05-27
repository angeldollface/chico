/*
CHICO by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// We import the data structure
/// for catching and handling errors.
use super::error::ChicoError;

/// Attempts to convert a binary number as a string
/// into a base-10 number and return it.
pub fn bin_to_dec(bin: &str) -> Result<u32, ChicoError> {
    let mut result: u32 = 0;
    let mut bin_chars: Vec<char> = bin.chars().collect();
    bin_chars.reverse();
    for (index, elem) in bin_chars.iter().enumerate() {
        if elem == &'1'{
            let power: u32 = match index.try_into(){
                Ok(power) => power,
                Err(e) => return Err::<u32, ChicoError>(ChicoError::new(&e.to_string()))
            };
            let to_add: u32 = 2_u32.pow(power);
            result += to_add;
        }
        else {}
    }
    Ok(result)
}

/// Converts a base-10 number 
/// to a binary number.
pub fn dec_to_bin(decimal: &u32) -> String{
    let mut im_result: Vec<String> = Vec::new();
    let mut dec: u32 = decimal.to_owned();
    im_result.push((dec%2).to_string());
    while dec / 2 != 0 {
      dec /= 2;
      let digit: String = (dec % 2).to_string();
      im_result.push(digit);
    }
    im_result.reverse();
    let result: String = im_result.join("");
    result
}

/// Checks whether the supplied
/// string is a binary number. Returns
/// a boolean depending on whether this is
/// the case or not.
pub fn is_bin(subject: &str) -> bool {
    let mut result: bool = true;
    let chars: Vec<char> = subject.chars().collect();
    for i in chars {
        if i == '1' || i == '0'{}
        else {
            result = false;
        }
    }
    result
}
