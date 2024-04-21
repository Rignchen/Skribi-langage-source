// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

use std::env;

use get_file_content::get_content;

// Import
use crate::tokens::tokenize;
use crate::utils::clear;

mod get_file_content;
mod interpret;
mod parse;
mod skr_errors;
mod tokenize;
mod tokens;
mod utils;

const FLAG_CHAR: &str = "--";

/// Launch the interpreter
fn main() {
    // parameters
    let extension: Vec<String> = vec!["skrb".to_string(), "skribi".to_string()];

    // generic parameters
    let args = env::args().collect::<Vec<_>>(); // get the command line arguments

    // clear the shell for the user
    if !args.contains(&format!("{FLAG_CHAR}compiler-debug")) {
        clear();
    }

    if let Ok(content) = get_content(args, extension) {
        // Read the file
        let lines = content;

        // Remove the comments and split the code into instructions
        match tokenize(lines) {
            Ok(tokens) => {
                let nodes = parse::main(tokens);
                // TODO
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    } else {
        panic!("Error while getting the content of the file. Check the file extension and the file path. Valid file extensions : {:?}", extension);
    }

    // Parse the code

    // interpret the code
    // interpret(code, args);
    /*let tokens = tokenize(content);*/

    // test
    /*for token in tokens {
        match token {
            Token::StringLiteral(string) => println!("StringLiteral: {}", string),
            Token::IntLiteral(int) => println!("IntLiteral: {}", int),
            Token::BooleanLiteral(boolean) => println!("BooleanLiteral: {}", boolean),
            _ => println!("{:?}", token),
        }
    }
    */
}
