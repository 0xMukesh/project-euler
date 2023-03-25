pub fn run() {
    let mut factors: Vec<i64> = Vec::new();
    let mut divisor: i64 = 2;
    let mut number: i64 = 600851475143;

    while number >= divisor {
        if number % divisor == 0 && !factors.contains(&divisor) {
            factors.push(divisor);
            number = number / divisor;
        } else {
            divisor += 1
        }
    }

    println!("{:?}", factors.iter().max().unwrap())
}
