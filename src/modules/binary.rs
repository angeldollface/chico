/*
CHICO by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// We import a function
/// to raise one number to another.
use super::utils::raise_to;

/// We import a wrapper around
/// the "string.split" method.
use super::utils::clean_split;

/// We import a function
/// to reverse a vector.
use super::utils::reverse_vec;

/// Attempts to convert a binary number as a string
/// into a base-10 number and return it.
pub fn bin_to_dec(bin: &String) -> u32 {
    let mut result: u32 = 0;
    let bin_chars: Vec<String> = clean_split(bin, &String::from(""));
    let reversed: Vec<String> = reverse_vec(&bin_chars);
    for (index, elem) in reversed.iter().enumerate() {
        if elem == &String::from("1") {
            let base: u32 = 2.try_into().unwrap();
            let power: u32 = index.try_into().unwrap();
            let to_add: u32 = raise_to(
                &base, &power
            );
            result += to_add;
        }
        else {
            // Do nothing.
        }
    }
    return result;
}

/// Converts a decimal number 
/// to a binary number.
pub fn dec_to_bin(decimal: &u32) -> String{
    let mut im_result: Vec<String> = Vec::new();
    let mut dec: u32 = decimal.to_owned();
    im_result.push((dec%2).to_string());
    while dec / 2 != 0 {
      dec = dec / 2;
      let digit: String = (dec % 2).to_string();
      im_result.push(digit);
    }
    let result: String = reverse_vec(&im_result).join("");
    return result;
}

/// Checks whether the supplied
/// string is a binary number. Returns
/// a boolean depending on if this is
/// the case or not.
pub fn is_bin(subject: &String) -> bool {
    let mut result: bool = true;
    let chars: Vec<String> = clean_split(
        subject, 
        &String::from("")
    );
    for i in chars {
        if i == String::from("1") || i == String::from("0"){
            // Do nothing.
        }
        else {
            result = false;
        }
    }
    return result;
}

