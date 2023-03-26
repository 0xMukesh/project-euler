mod p001;
mod p002;
mod p003;
mod p004;
mod p005;
mod p006;
mod p008;
mod p009;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        match (&args[1]).parse::<i32>() {
            Ok(i) => match i {
                1 => p001::run(),
                2 => p002::run(),
                3 => p003::run(),
                4 => p004::run(),
                5 => p005::run(),
                6 => p006::run(),
                8 => p008::run(),
                9 => p009::run(),
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
