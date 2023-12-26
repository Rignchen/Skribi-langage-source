// *-* coding:utf-8 *-*

////////////////////
// Skribi's shell //
////////////////////

mod interpret;
mod pre_run;

// Import
use interpret::main as interpret;
use pre_run::{get_instructions, get_path};
use skribi_language_source::{clear, read};
use std::env;

const FLAG_CHAR: &str = "/"; // if it was "-", it would sometimes interfere with cargo's flags

/**
 * Launch the interpreter
 */
fn main() {
    // parameters
    const EXTENSION: Vec<String> = vec!["skrb".to_string(), "skribi".to_string()];

    // generic parameters
    let args: Vec<_> = env::args().collect(); // get the command line arguments

    // clear the shell for the user
    if !args.contains(&String::from(FLAG_CHAR.to_string() + "interpret-debug")) {
        clear();
    }

    let path = get_path(args.clone(), FLAG_CHAR);

    // Check if the file has the right extension
    if !EXTENSION.contains(&String::from(path.split('.').last().unwrap())) {
        println!("Not a valid file extension");
        println!("Valid file extensions:");
        for ext in EXTENSION {
            println!("\t{}", ext);
        }
        return;
    }
    
    // Read the file
    let lines = read(&path);

    // Remove the comments and split the code into instructions
    let code = get_instructions(lines);

    // interpret the code
    interpret(code, args);
}
