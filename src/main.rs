mod add;
mod divide;
mod factorial;
mod multiply;
mod primes;
mod subtract;
use std::io;
use std::io::prelude::*;
use text_io::read;

fn main() {
    println!(r"                  _   _     _              _ ");
    println!(r"                 | | | |   | |            | |");
    println!(r"  _ __ ___   __ _| |_| |__ | |_ ___   ___ | |");
    println!(r" | '_ ` _ \ / _` | __| '_ \| __/ _ \ / _ \| |");
    println!(r" | | | | | | (_| | |_| | | | || (_) | (_) | |");
    println!(r" |_| |_| |_|\__,_|\__|_| |_|\__\___/ \___/|_|");
    println!(r"                                             ");
    loop {
    println!("Select a option:");
    println!("1: Add");
    println!("2: Divide");
    println!("3: Factorial");
    println!("4: Multiply");
    println!("5: Primes");
    println!("6: Subtract");
    let mut choice: i8 = read!();
    if choice == 1 {
        add::main();
    } else if choice == 2 {
        divide::main();
    } else if choice == 3 {
        factorial::main();
    } else if choice == 4 {
        multiply::main();
    } else if choice == 5 {
        primes::main();
    } else if choice == 6 {
        subtract::main();
    } else {
        println!("Invalid Option!");
    }
    }
}
