pub mod args {
    use super::super::randomgen::randomgen;
    use clap::Parser;
    use std::fs::OpenOptions;
    use std::io::Write;
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
        #[arg(short = 'n', long = "numbers")]
        pub include_numbers: bool,
        /// Include Symbols
        #[arg(short = 's', long = "symbols")]
        pub include_symbols: bool,
        /// Save generated output to a file
        #[arg(short = 'f', long = "file")]
        pub save_file: Option<String>,
        /// Do not print the generated password (use only when saving to a file)
        #[arg(long = "hide")]
        pub hide: bool,
    }

    pub fn parseargs() -> randomgen::Password {
        let arguments = Arguments::parse();
        let len = match arguments.length {
            Some(len) => {
                if len <= 0 {
                    eprintln!("The length of the password must be a positive integer");
                    process::exit(1);
                }
                len
            }

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

    pub fn save_pass(output: &String, path: Option<String>) {
        match path {
            Some(path) => {
                let sfile = OpenOptions::new()
                    .append(true)
                    .write(true)
                    .create(true)
                    .open(path);
                match sfile {
                    Ok(mut file) => {
                        if let Err(err) = writeln!(file, "{}", output) {
                            eprintln!("Error writing to file: {}", err);
                            process::exit(1);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error opening file: {}", e);
                        process::exit(1);
                    }
                }
            }
            None => {}
        }
    }
}
