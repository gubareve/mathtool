use std::io;
use std::io::prelude::*;
use text_io::read;

pub fn square(number: i128) -> i128 {
return number * number
}

fn main() {
    println!("Enter the first number!");
    let first_number: f32 = read!();
    println!("Enter the second number!");
    let second_number: f32 = read!();
    println!("The result is {}!", first_number/second_number);
}
