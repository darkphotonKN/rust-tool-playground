
mod my_funcs;
use crate::my_funcs::sum;
mod arrays;
use crate::arrays::addToArray;

fn main() {
    let mut x: u32 = 3;
    println!("mutable x before: {}", x);
    x = 2;
    println!("mutable x after: {}", x);

    // using sum function to add 1 and 3
    let y: u32 = sum(1, 3);
    println!("result: {}", y);

    // adding to a vector array
    let mut arr: Vec<u32> = vec![1,2,3]; // dynamic array 
    let mut arr2: Vec<u32>;

    arr2 = addToArray(4, arr);
    println!("arr2: {:?}", arr2);
    
}
