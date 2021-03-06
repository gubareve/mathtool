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
        println!("Enter the number of numbers to check!");
        i = read!();
    }
    let mut count = 0i128;
    let mut prime = true;
    for number_to_check in 2..i {
        prime = true;
        for i in 2..(number_to_check as f64).sqrt() as i32 + 1 {
            if number_to_check % i == 0 {
                // println!("{} is not a prime number.", number_to_check);
                // println!("{} times {} is {}.", i, number_to_check/i, number_to_check);
                prime = false;
                break;
            }
        }
        if prime {
            println!("{} is prime!", number_to_check);
        }
    }
}
