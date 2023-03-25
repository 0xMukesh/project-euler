mod p001;
mod p002;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        match (&args[1]).parse::<i32>() {
            Ok(i) => match i {
                1 => p001::init::run(),
                2 => p002::init::run(),
                _ => println!("Unknown problem number"),
            },
            Err(_) => {}
        }
    } else {
        println!("Invalid usage");
        println!("Usage - `cargo run <problem-number>`");
        println!("Example - `cargo run 1`")
    }
}
