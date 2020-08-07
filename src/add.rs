use std::io;
use std::io::prelude::*;
use text_io::read;
use std::env;

pub fn square(number: i128) -> i128 {
    return number * number;
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let mut first_number: i32 = 0;
    let mut second_number: i32 = 0;
    if args.len() == 3 {
      first_number = args[1].parse::<i32>().unwrap();
      second_number = args[2].parse::<i32>().unwrap();
    } else {
      println!("Enter the first number to add!");
      first_number = read!();
      println!("Enter the second number to add!");
      second_number = read!();
    }
    println!("The result is {}!", first_number + second_number);
}
