pub mod randomgen {
    use rand::Rng;

    pub struct Password {
        length: i8,
        has_letters: bool,
        has_caps: bool,
        has_number: bool,
        has_symbol: bool,
    }

    pub fn passinfo() -> Password {
        //? PASSWORD LENGTH

        println!("Password length:");
        let mut l = String::new();
        std::io::stdin().read_line(&mut l).unwrap();
        l = l.trim().parse().expect("Enter a number");
        let l = match l.parse::<i8>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Only numbers are allowed");
                0
            }
        };

        //? PASSWORD ARGS

        // LETTERS
        println!("Include Small letters? (y / n): ");
        let mut sl: String = String::new();
        std::io::stdin().read_line(&mut sl).unwrap();
        sl = sl.trim().parse().expect("Enter a char");
        let sl = match sl.parse::<char>() {
            Ok(char) => char,
            Err(_) => {
                eprintln!("Only characters are allowed");
                panic!("Char");
            }
        };
        let sl = if sl == 'y' {
            true
        } else if sl == 'n' {
            false
        } else {
            panic!("Y/N?");
        };


        // CAPS
        println!("Include Capital letters? (y / n): ");
        let mut c: String = String::new();
        std::io::stdin().read_line(&mut c).unwrap();
        c = c.trim().parse().expect("Enter a char");
        let c = match c.parse::<char>() {
            Ok(char) => char,
            Err(_) => {
                eprintln!("Only characters are allowed");
                panic!("Char");
            }
        };
        let c = if c == 'y' {
            true
        } else if c == 'n' {
            false
        } else {
            panic!("Y/N?");
        };

        // NUMS
        println!("Include Numbers? (y / n): ");
        let mut n: String = String::new();
        std::io::stdin().read_line(&mut n).unwrap();
        n = n.trim().parse().expect("Enter a char");
        let n = match n.parse::<char>() {
            Ok(char) => char,
            Err(_) => {
                eprintln!("Only numbers are allowed");
                panic!("Char");
            }
        };
        let n = if n == 'y' {
            true
        } else if n == 'n' {
            false
        } else {
            panic!("Y/N?");
        };

        // SYMBOL
        println!("Include Symbols? (y / n): ");
        let mut s: String = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s = s.trim().parse().expect("Enter a char");
        let s = match s.parse::<char>() {
            Ok(char) => char,
            Err(_) => {
                eprintln!("Only numbers are allowed");
                panic!("Char");
            }
        };
        let s = if s == 'y' {
            true
        } else if s == 'n' {
            false
        } else {
            panic!("Y/N?");
        };

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

        if !passinfo.has_caps && !passinfo.has_letters && !passinfo.has_number && !passinfo.has_symbol {
            panic!("Can't generate password that doesn't include anything!");
        }

        let mut loop_index = 0;
        loop {
            let gr = rng.gen_range(0..4);
            let charslen = abc.len();
            
            
            match gr {
                0 => {
                    if passinfo.has_letters {let alphar = rng.gen_range(0..charslen);
                        let letter = abc[alphar].to_ascii_lowercase().to_string();
                        pass.push(letter);
                        loop_index += 1;} else {continue;}
                    
                }
                1 => {
                    if passinfo.has_caps {
                    let alphar = rng.gen_range(0..charslen);
                    let letter = abc[alphar].to_ascii_uppercase().to_string();
                    pass.push(letter);
                    loop_index += 1;
                    } else {continue;}
                }
                2 => {
                    if passinfo.has_number {
                        let number = rng.gen_range(0..10).to_string();
                        pass.push(number);
                        loop_index +=1;
                    } else {continue;}
                }
                3 => {
                    if passinfo.has_symbol {
                        let symbr = rng.gen_range(0..symbols.len());
                        let symbol = symbols[symbr].to_string();
                        pass.push(symbol);
                        loop_index +=1;
                    } else {continue;}
                }
                _ => continue,
            }

            if loop_index == passinfo.length  {
                break;
            }
        }

        pass.concat()
    }
}
