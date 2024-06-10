/*
CHICO by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// We import the method to
/// check whether a string
/// is a binary number.
use super::binary::is_bin;

/// We import the method to
/// check whether a string
/// is an octal number.
use super::octal::is_octal;

/// We import the method to
/// convert a base-10 number
/// to a binary number.
use super::binary::dec_to_bin;

/// We import the method to
/// convert a binary number
/// to a base-10 number.
use super::binary::bin_to_dec;

/// We import the method to
/// check whether a string
/// is a hex number.
use super::hexadecimal::is_hex;

/// We import the method to
/// convert a hex number
/// to a base-10 number.
use super::hexadecimal::hex_to_dec;

/// We import the method to
/// convert a base-10 number
/// to a hex number.
use super::hexadecimal::dec_to_hex;

/// We import the method to
/// convert an octal number
/// to a base-10 number.
use super::octal::octal_to_dec;

/// We import the method to
/// convert a base-10 number to
/// an octal number.
use super::octal::dec_to_octal;

/// Tests the "bin_to_dec"
/// method from "./binary.rs".
#[test]
pub fn test_bin_to_dec(){
    let result: u32 = 5;
    match bin_to_dec("101"){
        Ok(num) => {
            assert_eq!(
                num,
                result
            );
        },
        Err(e) => {
            println!("{}", &e.to_string());
        }
    };
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

/// Tests the "is_bin"
/// method from "./binary.rs".
#[test]
pub fn test_is_bin(){
    assert_eq!(
        is_bin("1010101"),
        true
    );
    assert_eq!(
        is_bin("1010102"),
        false
    );
}

/// Tests the "hex_to_dec"
/// method from "./hexadecimal.rs".
#[test]
pub fn test_hex_to_dec(){
    let result: u32 = 42;
    match hex_to_dec("2A"){
        Ok(num) => {
            assert_eq!(
                num,
                result
            );
        },
        Err(e) => {
            println!("{}", &e.to_string())
        }
    }
}

/// Tests the "dec_to_hex"
/// method from "./hexadecimal.rs".
#[test]
pub fn test_dec_to_hex(){
    let init: u32 = 42;
    let result: String = String::from("2A");
    match dec_to_hex(&init){
        Ok(res) => {
            assert_eq!(
                res,
                result
            );
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}

/// Tests the "is_hex"
/// method from "./hexadecimal.rs".
#[test]
pub fn test_is_hex(){
    assert_eq!(
        is_hex("1A1A1A"),
        true
    );
    assert_eq!(
        is_hex("10101X"),
        false
    );
}

/// Tests the "is_octal"
/// method from "./octal.rs".
#[test]
pub fn test_is_octal(){
   assert_eq!(
        is_octal("151515"),
        true
    );
    assert_eq!(
        is_octal("171918"),
        false
    );
}

/// Tests the "octal_to_dec"
/// method from "./octal.rs".
#[test]
pub fn test_octal_to_dec(){
    let result: u32 = 885;
    match octal_to_dec("1565"){
        Ok(num) => {
            assert_eq!(
                num,
                result
            );
        },
        Err(e) => {
            println!("{}", &e.to_string())
        }
    }
}

/// Tests the "dec_to_octal"
/// function from "./octal.rs".
#[test]
pub fn test_dec_to_octal(){
    let num: &str = "16";
    let result: String = "20".to_string();
    match dec_to_octal(num){
        Ok(res) => {
            assert_eq!(res, result);
        },
        Err(e) => {
            println!("{}", &e.to_string());
        }
    };
}
