use std::env;
use std::io;
use std::io::prelude::*;
use text_io::read;

pub fn square(number: i128) -> i128 {
    return number * number;
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let mut i: i32 = 0;
    if args.len() == 2 {
        i = args[1].parse::<i32>().unwrap();
    } else {
        println!("Enter the number to find the factorial of!");
        i = read!();
    }
    let times_to_multiply = i + 1;
    let mut result: i32 = 1;
    for to_multiply in 1..times_to_multiply {
        result = result * to_multiply;
    }
    println!("The result is {}!", result);
}
