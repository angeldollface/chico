/*
CHICO by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Returns a vector of strings from a character split for a string.
/// Both the string and split character have to be strings.
pub fn clean_split(subject: &String, split_char: &String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for i in subject.split(split_char) {
        if i == split_char {
            // Do nothing.
        }
        else {
            result.push(i.to_string());
        }
    }
    return result;
}

/// Checks if a string is a number or not.
/// Returns a boolean based on the result.
pub fn is_num(char: &String) -> bool {
    let mut result: bool = false;
    let match_op = char.parse::<usize>();
    match match_op {
        Ok(_x) => {
            result = true;
        },
        Err(_e) => {}
    };
    return result;
}

/// Converts string to a number.
/// Returns zero if the conversion cannot
/// be completed.
pub fn conv_to_num(char: &String) -> usize {
    let mut result: usize = 0;
    if is_num(&char) == true {
        result = char.parse::<usize>().unwrap();
    }
    else {}
    return result;
}

/// Raises to "base" to the power of "power" and
/// returns the result.
pub fn raise_to(base: &u32, power: &u32) -> u32 {
    return base.pow(*power);
}

/// Reverses the order of a vector and 
/// returns the reversed vector.
pub fn reverse_vec<T: Clone>(subject: &Vec<T>) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();
    let last_index: usize = subject.len();
    for i in (0..last_index).rev() {
        result.push(subject[i].clone());
    }
    return result;
}