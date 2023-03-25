// Sum of squares of first n natural numbers is given by [n(n+1)(2n+1)]/6
// Sum of first n natural numbers is given by [n(n+1)]/2

pub fn run() {
    const LIMIT: i32 = 100;
    let sum_of_squares = (LIMIT * (LIMIT + 1) * (2 * LIMIT + 1)) / 6;
    let square_of_sum = ((LIMIT * (LIMIT + 1)) / 2).pow(2);
    let differance = square_of_sum - sum_of_squares;
    println!("{}", differance)
}
