use num::integer::lcm;

// The number N which is evenly divisible among a set of number {k1, k2, k3, ..., kn} is called as LCM of that set of numbers
// LCM(k1, k2, k3, ..., kn) = LCM(...(LCM(LCM(k1, k2), k3)...), kn))
// LCM of three numbers x, y and z is given as LCM(LCM(x, y), z)

pub fn run() {
    let mut num = 1;
    for i in 1..=20 {
        num = lcm(num, i)
    }
    println!("{}", num);
}
