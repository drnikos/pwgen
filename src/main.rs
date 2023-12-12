mod randomgen;
use randomgen::randomgen::{passgen, passinfo};
fn main() {
    let x = passinfo();
    println!("Your password is: \x1b[93m{}\x1b[0m", passgen(x));
}