/*
CHICO by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// Importing the main
/// Cliply API struct.
use cliply::App;

/// Importing the function to
/// check whether a string
/// is a binary number or not.
use super::binary::is_bin;

/// Importing the function
/// to check whether a string
/// is an octal number or not.
use super::octal::is_octal;

/// We import the error structure
/// to handle errors.
use super::error::ChicoError;

/// We import the function
/// to convert a base-10 number
/// to a base-2 number.
use super::binary::dec_to_bin;

/// We import the method to
/// convert a binary number
/// to a base-10 number.
use super::binary::bin_to_dec;

/// We import the method to
/// convert an octal number to a
/// base-10 number.
use super::octal::octal_to_dec;

/// Importing the function to check whether
/// a string is a hexadecimal number or not.
use super::hexadecimal::is_hex;

/// We import the function
/// to convert a base-10 number
/// to a base-16 number.
use super::hexadecimal::dec_to_hex;

/// We import the method to
/// convert a hex number
/// to a base-10 number.
use super::hexadecimal::hex_to_dec;


/// CLI for Chico.
pub fn cli() -> Result<String, ChicoError> {

    // Instantiating the "App" struct with the required
    // data.
    let mut app: App = App::new(
        "Chico",
        "0.3.0",
        "Angel Dollface"
    );

    let err_msg: String = "No integer provided.".to_string();

    // The flag to supply Chico with base-2 numbers.
    app.add_arg(
        "bin",
        "      binary number to accept", 
        "true"
    );

    // The flag to supply Chico with base-16 numbers.
    app.add_arg(
        "hex",
        "      hex number to accept", 
        "true"
    );

    app.add_arg(
        "oct",
        "      octal number to accept",
        "true"
    );

    app.add_arg(
        "dec",
        "      base-10 number to accept",
        "true"
    );

    app.add_arg(
        "xoc",
        "     convert a base-10 number to a hex number",
        "false"
    );

    app.add_arg(
        "yoc",
        "     convert a base-10 number to a binary number",
        "false"
    );

    app.add_arg(
        "cto",
        "     convert a base-10 number to an octal number",
        "false"
    );


    // Was the version flag used?
    if app.version_is() {
        Ok(app.version_info())
    }

    // Was the help flag used?
    else if app.help_is() {
        Ok(app.help_info())
    }

    // Was a binary number supplied?
    else if app.arg_was_used("bin") {
        let arg_data: String = match app.get_arg_data("bin") {
            Ok(arg_data) => arg_data,
            Err(e) => return Err::<String, ChicoError>(ChicoError::new(&e.to_string()))
        };
        if is_bin(&arg_data){
            let converted: u32 = match bin_to_dec(&arg_data){
                Ok(converted) => converted,
                Err(e) => return Err::<String, ChicoError>(ChicoError::new(&e.to_string()))
            };
            Ok(converted.to_string())
        }
        else {
            let e: String = format!("\"{}\" is not a binary number.", &arg_data);
            return Err::<String, ChicoError>(ChicoError::new(&e.to_string()));
        }
    }

    // Was a hex number supplied?
    else if app.arg_was_used("hex") {
        let arg_data: String = match app.get_arg_data("hex"){
            Ok(arg_data) => arg_data,
            Err(e) => return Err::<String, ChicoError>(ChicoError::new(&e.to_string()))
        };
        if is_hex(&arg_data){
            let res: u32 = match hex_to_dec(&arg_data){
                Ok(res) => res,
                Err(e) => return Err::<String, ChicoError>(ChicoError::new(&e.to_string()))
            };
            Ok(res.to_string())
        }
        else {
            let e: String = format!("\"{}\" is not a hexadecimal number.", &arg_data);
            return Err::<String, ChicoError>(ChicoError::new(&e.to_string()));
        }
    }

    else if app.arg_was_used("oct"){
        let arg_data: String = match app.get_arg_data("oct"){
            Ok(arg_data) => arg_data,
            Err(e) => return Err::<String, ChicoError>(ChicoError::new(&e.to_string()))
        };
        if is_octal(&arg_data){
            let res: u32 = match octal_to_dec(&arg_data){
                Ok(res) => res,
                Err(e) => return Err::<String, ChicoError>(ChicoError::new(&e.to_string()))
            };
             Ok(res.to_string())
        }
        else {
            let e: String = format!("\"{}\" is not an octal number.", &arg_data);
            return Err::<String, ChicoError>(ChicoError::new(&e.to_string()));
        }
    }

    else if app.arg_was_used("xoc") && app.arg_was_used("dec"){
        let arg_data: String = match app.get_arg_data("dec"){
            Ok(arg_data) => arg_data,
            Err(e) => return Err::<String, ChicoError>(ChicoError::new(&e.to_string()))
        };
        let subject: u32 = match arg_data.parse(){
            Ok(subject) => subject,
            Err(e) => return Err::<String, ChicoError>(ChicoError::new(&e.to_string()))
        };
        let res: String = match dec_to_hex(&subject){
            Ok(res) => res,
            Err(_e) => return Err::<String, ChicoError>(ChicoError::new(&err_msg))
        };
        Ok(res)

    }

    else if app.arg_was_used("yoc") && app.arg_was_used("dec"){
        let arg_data: String = match app.get_arg_data("dec"){
            Ok(arg_data) => arg_data,
            Err(e) => return Err::<String, ChicoError>(ChicoError::new(&e.to_string()))
        };
        let subject: u32 = match arg_data.parse(){
            Ok(subject) => subject,
            Err(_e) => return Err::<String, ChicoError>(ChicoError::new(&err_msg))
        };
        let res: String = dec_to_bin(&subject);
        Ok(res)
    }

    else if app.arg_was_used("cto") && app.arg_was_used("dec"){
        let arg_data: String = match app.get_arg_data("dec"){
            Ok(arg_data) => arg_data,
            Err(e) => return Err::<String, ChicoError>(ChicoError::new(&e.to_string()))
        };
        let subject: u32 = match arg_data.parse(){
            Ok(subject) => subject,
            Err(_e) => return Err::<String, ChicoError>(ChicoError::new(&err_msg))
        };
        // TO DO: implement dec to octal
        //Ok(subject.to_string())
        Ok(subject.to_string())
    }

    // If user-supplied flags are invalid, show
    // them the app's help message.
    else {
        Ok(app.help_info())
    }
}
