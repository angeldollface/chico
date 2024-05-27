/*
CHICO by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// We import the data structure
/// for catching and handling errors.
use super::error::ChicoError;

/// Attempts to convert a hexadecimal number in
/// string format to a base-10 number.
pub fn hex_to_dec(hex: &str) -> Result<u32, ChicoError> {
    let mut result: u32 = 0;
    let mut hex_num_chars: Vec<char> = hex.chars().collect();
    hex_num_chars.reverse();
    let hex_alphabet: Vec<char> = "123456789ABCDEF".to_string().chars().collect();
    for (index, character) in hex_num_chars.into_iter().enumerate(){
        let num = match hex_alphabet.iter().position(|&r| r == character){
            Some(num) => num,
            None => {
                let e: String = "Invalid characters found.".to_string();
                return Err::<u32,ChicoError>(ChicoError::new(&e.to_string()));
            }
        };
        let idx: u32 = index as u32;
        let multiplier: u32 = (num as u32) + 1;
        let to_add = (16_u32.pow(idx)) * multiplier;
        result += to_add;
    }
    Ok(result)
}

/// Converts a base-10 number to a hexadecimal number.
pub fn dec_to_hex(decimal: &u32) -> Result<String, ChicoError> {
    let result: String;
    let base: u32 = 16;
    let mut nums: Vec<String> = Vec::new();
    let alphabet: Vec<char> = "123456789ABCDEF".to_string().chars().collect();
    let mut digit: u32;
    let mut remainder: u32 = base + 1;
    while remainder > base {
        digit = decimal / base;
        remainder = decimal - (digit * base);
        if digit > base {
            nums.push(digit.to_string());
        } 
        else {
            if digit > 9 {
                let idx: usize = match <u32 as TryInto<usize>>::try_into(digit){
                    Ok(to_push) => to_push,
                    Err(e) => return Err::<String,ChicoError>(ChicoError::new(&e.to_string()))
                };
                nums.push((alphabet[idx-1]).to_string());
            } 
            else {
                nums.push(digit.to_string());
            }
        }
        if remainder > base {
        } 
        else {
            let idx: usize = match <u32 as TryInto<usize>>::try_into(remainder){
                Ok(idx) => idx,
                Err(e) => return Err::<String,ChicoError>(ChicoError::new(&e.to_string()))
            };
            nums.push((alphabet[idx - 1]).to_string());
            break;
        }
    }
    result = nums.join("");
    Ok(result)
}

/// Checks whether the supplied
/// string is a hexadecimal number. Returns
/// a boolean depending on whether this is
/// the case or not.
pub fn is_hex(subject: &str) -> bool {
    let mut result: bool = true;
    let alphabet: Vec<char> = "123456789ABCDEF".to_string().chars().collect();
    let chars: Vec<char> = subject.chars().collect();
    for i in chars{
        if alphabet.contains(&i){}
        else {
            result = false;
        }
    }
    result
}
