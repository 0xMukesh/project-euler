use std::fs::read_to_string;
use substring::Substring;

const CHUNK_SIZE: usize = 13;

pub fn run() {
    let content: Result<String, _> = read_to_string("data/p008.txt").expect("not found").parse();
    let number = content.unwrap();
    let range = 0..=number.len() - CHUNK_SIZE;
    let (mut largest, mut pair): (u64, &str) = (1, "");

    for i in range {
        let substring = number.substring(i, i + CHUNK_SIZE);
        let chunk: &Vec<char> = &substring.chars().collect();
        let mut product: u64 = 1;

        for j in chunk {
            let digit: u64 = j.to_digit(10).unwrap().into();
            product *= digit;
        }

        if product > largest {
            largest = product;
            pair = substring;
        }
    }

    println!("product - {}", largest);
    println!("pair - {}", pair);
}
