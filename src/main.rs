mod args;
mod randomgen;
use crate::args::args::{parseargs, save_pass};
use clap::Parser;
use randomgen::randomgen::{passgen, passinfo};
use std::process;
use std::io;
use std::io::prelude::*;


fn main() {
    let arguments = args::args::Arguments::parse();

    if arguments.include_caps
        || arguments.include_numbers
        || arguments.include_symbols
        || arguments.include_letters
    {
        if arguments.length.is_some() {
            let password = parseargs();
            let password = passgen(password);
            save_pass(&password, arguments.save_file);
            if !arguments.hide {
                println!("Your password is: \x1b[93m{}\x1b[0m", password);
            }
        } else {
            eprintln!("Cannot generate password if the length is not provided");
            process::exit(1);
        }
        process::exit(0);
    } else if arguments.length.is_some()
        && !arguments.include_caps
        && !arguments.include_numbers
        && !arguments.include_symbols
        && !arguments.include_letters
    {
        eprintln!("Cannot generate password without any type of characters");
        process::exit(1);
    }
    let password = passinfo();
    println!("Your password is: \x1b[93m{}\x1b[0m", passgen(password));

    pause();
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "\n\nPress any key to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}