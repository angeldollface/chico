/*
CHICO by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// Returns a vector of strings from a character.
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

/// Raises the "base" to the power of "power" and
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