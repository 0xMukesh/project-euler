pub fn run() {
    let mut current = 1;
    let mut next = 2;
    let mut even_sum = 0;

    loop {
        let sum = current + next;
        current = next;
        next = sum;

        if current % 2 == 0 {
            even_sum += current;
        }

        if current > 4_000_000 {
            break;
        }
    }

    println!("{}", even_sum)
}
