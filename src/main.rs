
mod my_funcs;
use crate::my_funcs::sum;

fn main() {
    let mut x: u32 = 3;
    println!("mutable x before: {}", x);
    x = 2;
    println!("mutable x after: {}", x);

    // using sum function to add 1 and 3
    let y: u32 = sum(1, 3);
    println!("result: {}", y);
}
