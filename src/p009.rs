// The sum of three sides of a triangle is called as its perimeter
// The length of any one side can't be greater than or equal to perimeter

const PERIMETER: i32 = 1000;

pub fn run() {
    'outer_loop: for a in 1..=PERIMETER {
        // As a < b < c, no side is equal to another side
        for b in a + 1..PERIMETER + 1 {
            let c = PERIMETER - (a + b);
            let lhs = a.pow(2) + b.pow(2);
            let rhs = c.pow(2);

            if lhs == rhs {
                println!("{}, {}, {}", a, b, c);
                println!("product - {}", a * b * c);
                break 'outer_loop;
            }
        }
    }
}
