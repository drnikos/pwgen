mod args;
mod randomgen;
use randomgen::randomgen::{passgen, passinfo};
use clap::Parser;
use crate::args::args::parseargs;

fn main() {
    let arguments = args::args::Arguments::parse();
    if let Some(_) = arguments.length {
        let password = parseargs();
        println!("Your password is: \x1b[93m{}\x1b[0m", passgen(password));
    } else { 
        let password = passinfo();
        println!("Your password is: \x1b[93m{}\x1b[0m", passgen(password));
    }
}
