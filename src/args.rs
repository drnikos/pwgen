pub mod args {
    use super::super::randomgen::randomgen;
    use clap::Parser;
    use std::process;

    #[derive(Parser)]
    #[command(
        author,
        version,
        about = "A simple Password Generator CLI tool written in Rust"
    )]
    pub struct Arguments {
        /// Length of the password
        #[arg(short = 'l', long = "length")]
        pub length: Option<i64>,
        /// Include Small letters
        #[arg(short = 'm', long = "small")]
        pub include_letters: bool,
        /// Include Capital letters
        #[arg(short = 'c', long = "caps")]
        pub include_caps: bool,
        /// Include Numbers
        #[arg(short = 'n', long = "number")]
        pub include_numbers: bool,
        // Include Symbols
        #[arg(short = 's', long = "symbol")]
        pub include_symbols: bool,
    }

    pub fn parseargs() -> randomgen::Password {
        let arguments = Arguments::parse();
        let len = match arguments.length {
            Some(len) => {
                if len <= 0 {
                    eprintln!("The length of the password cannot be 0");
                    process::exit(1);
                }
                len
            },
            
            None => {
                eprintln!("Invalid length");
                process::exit(1)
            }
        };

        if len > 0
            && !arguments.include_caps
            && !arguments.include_numbers
            && !arguments.include_symbols
            && !arguments.include_letters
        {
            eprintln!("Cannot generate password that does not contain any type of characters");
            process::exit(1)
        }

        randomgen::Password {
            length: len,
            has_letters: arguments.include_letters,
            has_caps: arguments.include_caps,
            has_number: arguments.include_numbers,
            has_symbol: arguments.include_symbols,
        }
    }
}
