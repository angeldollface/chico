/*
CHICO by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// Importing the main
/// Cliply API struct.
use cliply::App;

/// We import the method to
/// convert a binary number
/// to a base-10 number.
use super::binary::bin_to_dec;

/// We import the method to
/// convert a hex number
/// to a base-10 number.
use super::hexadecimal::hex_to_dec;

/// CLI for Chico.
pub fn cli() -> () {

    // Instantiating the "App" struct with the required
    // data.
    let mut app: App = App::new(
        &"Chico",
        &"0.2.0",
        &"Angel Dollface"
    );

    // The flag to supply Chico with base-2 numbers.
    app.add_arg(
        &"bin",
        &"      binary number to accept", 
        &"true"
    );

    // The flag to supply Chico with base-16 numbers.
    app.add_arg(
        &"otl",
        &"      hex number to accept", 
        &"true"
    );

    // Was the version flag used?
    if app.version_is() {
        println!("{}", app.version_info());
    }

    // Was the help flag used?
    else if app.help_is() {
        println!("{}", app.help_info());
    }

    // Was a binary number supplied?
    else if app.arg_was_used(&"bin") {
        let arg_data = app.get_arg_data(&"bin");
        match arg_data{
            Ok(arg_data) => {
                match bin_to_dec(&arg_data){
                    Ok(num) => {
                        println!("{}", num.to_string());
                    },
                    Err(conv_error) => {
                        println!("{}", &conv_error.to_string());
                    }
                }
            },
            Err(e) => {
                println!("{}", &e.to_string());
            }
        };
    }

    // Was a hex number supplied?
    else if app.arg_was_used(&"nex") {
        let arg_data = app.get_arg_data(&"nex");
        match arg_data{
            Ok(arg_data) => {
                match hex_to_dec(&arg_data){
                    Ok(num) => {
                        println!("{}", num.to_string());
                    },
                    Err(conv_error) => {
                        println!("{}", &conv_error.to_string());
                    }
                }
            },
            Err(e) => {
                println!("{}", &e.to_string());
            }
        };
    }

    // If user-supplied flags are invalid, show
    // them the app's help message.
    else {
        println!("{}", app.help_info());
    }
}