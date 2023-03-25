mod p001;
mod p002;
mod p003;
mod p004;
mod p005;
mod p006;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        match (&args[1]).parse::<i32>() {
            Ok(i) => match i {
                1 => p001::init::run(),
                2 => p002::init::run(),
                3 => p003::init::run(),
                4 => p004::init::run(),
                5 => p005::init::run(),
                6 => p006::init::run(),
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
