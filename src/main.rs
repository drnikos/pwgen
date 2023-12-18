mod args;
mod randomgen;
use crate::args::args::parseargs;
use clap::Parser;
use randomgen::randomgen::{passgen, passinfo};
use std::process;

fn main() {
    let arguments = args::args::Arguments::parse();

    if arguments.include_caps
        || arguments.include_numbers
        || arguments.include_symbols
        || arguments.include_letters
    {
        if let Some(_) = arguments.length {
            let password = parseargs();
            println!("Your password is: \x1b[93m{}\x1b[0m", passgen(password));
        } else {
            eprintln!("Cannot generate password if the length is not provided");
            process::exit(1);
        }
    } else if arguments.length.is_some()
        && !arguments.include_caps
        && !arguments.include_numbers
        && !arguments.include_symbols
        && !arguments.include_letters
    {
        eprintln!("Cannot generate password without any type of characters");
        process::exit(1);
    } else {
        let password = passinfo();
        println!("Your password is: \x1b[93m{}\x1b[0m", passgen(password));
    }
}
