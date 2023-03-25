pub fn run() {
    let mut largest = 0;

    for i in 100..=999 {
        for j in 100..=999 {
            let product = i * j;
            let reversed: String = format!("{}", product).chars().rev().collect();

            if product > largest && reversed == format!("{}", product) {
                largest = product
            }
        }
    }

    println!("{}", largest)
}
