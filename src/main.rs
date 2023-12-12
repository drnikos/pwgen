mod randomgen;
use randomgen::randomgen::{passgen, passinfo};
fn main() {
    let x = passinfo();
    println!("Your password is: {}", passgen(x));
}