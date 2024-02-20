use std::io::{self, Write};
use rand::prelude::*;
pub mod read;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut trycount = 10;
    let (mut _start, mut _end) = (String::new(), String::new());
    for arg in args {
        if arg.starts_with("-t") {
            trycount = read::read_i32(&arg[2..].to_string());
        }
        if arg.starts_with("-s") {
            _start = arg.strip_prefix("-s").unwrap().to_string();
        }
        if arg.starts_with("-e") {
            _end = arg.strip_prefix("-e").unwrap().to_string();
        }
    }
    println!("The following program will choose a number between the numbers you supply.");
    if _start.is_empty() {
        print!("Please supply a starting number: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut _start).unwrap();
    }
    let start = _start.trim().parse::<i32>().unwrap();
    if _end.is_empty() {
        print!("Please supply an ending number: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut _end).unwrap();
    }
    let end = _end.trim().parse::<i32>().unwrap();
    let rand = thread_rng().gen_range(start..end);
    println!("\nYou have {} tries to guess the number.\n", trycount);
    io::stdout().flush().unwrap();
    print!("Pick a number between {} and {}: ", start, end);
    for i in 0..trycount {
        if i != 0 { print!("Try again: "); }
        let mut value = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut value).unwrap();
        let val = read::read_i32(&value);
        if val > rand {
            println!("Too high! {} tr{} remaining.\n", trycount - i - 1, if trycount - i == 2 { "y" } else { "ies" });
        }
        if val < rand {
            println!("Too low! {} tr{} remaining.\n", trycount - i - 1, if trycount - i == 2 { "y" } else { "ies" });
        }
        if val == rand {
            println!("You got it in {} tr{}!\n", i + 1, if i == 0 { "y" } else { "ies" });
            break;
        }
    }
    println!("The number was {}!", rand);
}
