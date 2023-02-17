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

/// Converts a hexadecimal number in
/// string format to a base-10 number.
pub fn hex_to_dec(hex: &String) -> u32 {
    let base: u32 = 16;
    let mut result: u32 = 0;
    let digit_set: Vec<String> = clean_split(
        &String::from("0123456789ABCDEF"),
        &String::from("")
    );
    let subject_set: Vec<String> = reverse_vec(
        &clean_split(
            hex,
            &String::from("")
        )
    );
    for (index, element) in subject_set.iter().enumerate() {
        let mut im_index: u32 = 0;
        for (digit_index, digit) in digit_set.iter().enumerate() {
            if element == digit {
                im_index = digit_index.try_into().unwrap();
            }
            else {
                // Do nothing.
            }
        }
        let pos_power: u32 = raise_to(&base, &index.try_into().unwrap());
        let to_add: u32 = im_index * pos_power;
        result += to_add;
        im_index = 0;
    }
    return result;
}

/// Converts a decimal number to a hexadecimal number.
pub fn dec_to_hex(decimal: &u32) -> String {
    let mut result: String = String::from("");
    let base: u32 = 16;
    let mut nums: Vec<String> = Vec::new();
    let alphabet: Vec<String> = clean_split(
        &String::from("123456789ABCDEF"),
        &String::from("")
    );
    let mut digit: u32 = 0;
    let mut remainder: u32 = base + 1;
    while remainder > base {
        digit = decimal / base;
        remainder = decimal - (digit * base);
        if digit > base {
            nums.push(digit.to_string());
        } 
        else {
            if digit > 9 {
                nums.push(alphabet[<u32 as TryInto<usize>>::try_into(digit).unwrap() - 1].clone());
            } else {
                nums.push(digit.to_string());
            }
        }
        if remainder > base {
        } 
        else {
            nums.push(alphabet[<u32 as TryInto<usize>>::try_into(remainder).unwrap() - 1].clone());
            break;
        }
    }
    result = nums.join("");
    return result;
}

/// Checks whether the supplied
/// string is a hexadecimal number. Returns
/// a boolean depending on if this is
/// the case or not.
pub fn is_hex(subject: &String) -> bool {
    let mut result: bool = true;
    let alphabet: Vec<String> = clean_split(
        &String::from("123456789ABCDEF"),
        &String::from("")
    );
    let chars: Vec<String> = clean_split(
        subject, 
        &String::from("")
    );
    if chars.len() != 6 {
        result = false;
    }
    else {
        for i in chars{
            if alphabet.contains(&i){
                // Do nothing.
            }
            else {
                result = false;
            }
        }
    }
    return result;
}