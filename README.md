# PWGEN 
<p> A simple Password Generator tool written in Rust.

**CURRENTLY, ONLY LINUX IS SUPPORTED!**

## Usage
**Make sure pwgen is excecutable by running:** ```chmod +x pwgen``` **in the directory the file is located**
<p> The program can be run either by passing arguments or by excecuting it directly

#### Direct Excecution
![231219_00h25m41s_screenshot](https://github.com/drnikos/pwgen/assets/153459342/cd75426c-6592-4c6c-a9a0-36bb9ea8d582)

#### With arguments
![231219_00h29m17s_screenshot](https://github.com/drnikos/pwgen/assets/153459342/62a303d6-233f-459b-bbfb-397fb1b439b2)

## Examples
By running ```./pwgen -l 12 -m -n -s  -f ~/output.txt --hide```, the program generates a password with 12 characters, that includes small letters, numbers, and symbols, and saves the output to **~/output.txt** (Appends it if the file already exists, or creates the file if it doesn't), without printing it to the console.


---
For more information, run: ```./pwgen -h```
---