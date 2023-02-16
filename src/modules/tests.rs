/*
CHICO by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// We import all
/// methods from
/// the "./utils.rs"
/// module.
use super::utils::*;

/// We import all
/// methods from
/// the "./binary.rs"
/// module.
use super::binary::*;

/// We import all
/// methods from
/// the "./hexadecimal.rs"
/// module.
use super::hexadecimal::*;

/// Tests the "clean_split" method from
/// "./utils.rs".
#[test]
pub fn test_split() {
    assert_eq!(
        clean_split(&String::from("101"), &String::from("")),
        vec![
            String::from("1"), 
            String::from("0"), 
            String::from("1")
        ]
    );
}

/// Tests the "is_num" method from
/// "./utils.rs".
#[test]
pub fn test_num_check() {
    assert_eq!(
        is_num(&String::from("1")),
        true
    );
    assert_eq!(
        is_num(&String::from("A")),
        false
    );
}

/// Tests the "conv_to_num"
/// method from "./utils.rs".
#[test]
pub fn test_num() {
    let planned_result: usize = 4;
    let false_result: usize = 0;
    assert_eq!(
        conv_to_num(&String::from("4")),
        planned_result
    );
    assert_eq!(
        conv_to_num(&String::from("A")),
        false_result
    );
}

/// Tests the "raise_to"
/// method from "./utils.rs".
#[test]
pub fn test_power() {
    let base: u32 = 2;
    assert_eq!(
        raise_to(&base, &base),
        4
    );
}

/// Tests the "reverse_vec"
/// method from "./utils.rs".
#[test]
pub fn test_reverse(){
    let test_vec: Vec<usize> = vec![0,1,2];
    let result_vec: Vec<usize> = vec![2,1,0];
    assert_eq!(
        reverse_vec(&test_vec),
        result_vec
    );
}

/// Tests the "bin_to_dec"
/// method from "./binary.rs".
#[test]
pub fn test_bin_to_dec(){
    let result: u32 = 5;
    bin_to_dec(&String::from("101"));
    assert_eq!(
        bin_to_dec(&String::from("101")),
        result
    );
}

/// Tests the "dec_to_bin"
/// method from "./binary.rs".
#[test]
pub fn test_dec_to_bin(){
    let result: String = String::from("101");
    let init: u32 = 5;
    assert_eq!(
        dec_to_bin(&init),
        result
    );
}

/// Tests the "hex_to_dec"
/// method from "./hexadecimal.rs".
#[test]
pub fn test_hex_to_dec(){
    let hex: String = String::from("2A");
    let result: u32 = 42;
    assert_eq!(
        hex_to_dec(&hex),
        result
    );
}

/// Tests the "dec_to_hex"
/// method from "./hexadecimal.rs".
#[test]
pub fn test_dec_to_hex(){
    let init: u32 = 42;
    let result: String = String::from("2A");
    assert_eq!(
        dec_to_hex(&init),
        result
    );
}

