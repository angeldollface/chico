/*
CHICO by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// Importing Chico's
/// CLI.
use chico::cli;

/// Importing functions
/// to provide an exit code.
use std::process::exit;

/// The main point
/// of entry for the
/// Rust compiler.
fn main(){
    match cli(){
        Ok(res) => {
            println!("{}", res);
            exit(0);
        },
        Err(e) => {
            eprintln!("{}", &e.to_string());
            exit(1);
        }
    };
}
