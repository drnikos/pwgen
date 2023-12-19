pub mod randomgen {
    use rand::Rng;
    use std::process;

    pub struct Password {
        pub length: i64,
        pub has_letters: bool,
        pub has_caps: bool,
        pub has_number: bool,
        pub has_symbol: bool,
    }

    fn user_input(prompt: &str) -> String {
        println!("{}", prompt);
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().parse().expect("Error reading input");

        input.trim().to_string()
    }
    fn yn(prompt: &str) -> bool {
        loop {
            let answer = user_input(&prompt).to_lowercase();
            match answer.as_str() {
                "y" => return true,
                "n" => return false,
                _ => println!("Please enter 'y' or 'n'."),
            }
        }
    }
    pub fn passinfo() -> Password {
        //? PASSWORD LENGTH
        let l = loop {
            let len = user_input("Password length: ");
            match len.parse::<i64>() {
                Ok(num) => {
                    if num <= 0 {
                        println!("The length of the password cannot be 0");
                        continue;
                    }
                    break num;
                }
                Err(_) => {
                    println!("Please enter a positive integer.");
                    continue;
                }
            };
        };

        //? PASSWORD ARGS

        let sl = yn("Include Small letters? (y / n): ");
        let c = yn("Include Capital letters? (y / n): ");
        let n = yn("Include Numbers? (y / n): ");
        let s = yn("Include Symbols? (y / n): ");

        Password {
            length: l,
            has_letters: sl,
            has_number: n,
            has_symbol: s,
            has_caps: c,
        }
    }

    pub fn passgen(passinfo: Password) -> String {
        let mut rng = rand::thread_rng();
        let mut pass: Vec<String> = Vec::new();

        let symbols = vec![
            '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', ',', '.', '/', '?', '`', '~', '-',
            '_', '=', '+',
        ];
        let abc = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'o', 'p', 'q', 'r',
            's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];

        if !passinfo.has_caps
            && !passinfo.has_letters
            && !passinfo.has_number
            && !passinfo.has_symbol
        {
            eprintln!("Can't generate password that doesn't include anything!");
            process::exit(1);
        }

        let mut loop_index = 0;
        loop {
            let gr = rng.gen_range(0..4);
            let charslen = abc.len();

            match gr {
                0 => {
                    if passinfo.has_letters {
                        let alphar = rng.gen_range(0..charslen);
                        let letter = abc[alphar].to_ascii_lowercase().to_string();
                        pass.push(letter);
                        loop_index += 1;
                    }
                }
                1 => {
                    if passinfo.has_caps {
                        let alphar = rng.gen_range(0..charslen);
                        let letter = abc[alphar].to_ascii_uppercase().to_string();
                        pass.push(letter);
                        loop_index += 1;
                    }
                }
                2 => {
                    if passinfo.has_number {
                        let number = rng.gen_range(0..10).to_string();
                        pass.push(number);
                        loop_index += 1;
                    }
                }
                3 => {
                    if passinfo.has_symbol {
                        let symbr = rng.gen_range(0..symbols.len());
                        let symbol = symbols[symbr].to_string();
                        pass.push(symbol);
                        loop_index += 1;
                    }
                }
                _ => continue,
            }

            if loop_index == passinfo.length {
                break;
            }
        }

        pass.concat()
    }
}
