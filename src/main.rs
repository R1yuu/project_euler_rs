// Multiples of 3 or 5
fn problem_1() {
    let mut sum: u32 = 0;
    for x in (3..1000).step_by(3) {
        sum += x;
    }
    for y in (5..1000).step_by(5) {
        sum += if y % 15 == 0 { 0 } else { y };
    }

    println!("{}", sum);
}

// Even Fibonacci numbers
fn problem_2() {
    let mut x:      u32 = 1;
    let mut y:      u32 = 2;
    let mut tmp:    u32;
    let mut sum = 0;
    while x < 4000000 && y < 4000000 {
        if y % 2 == 0 {
            sum += y;
        }
        tmp = x + y;
        x = y;
        y = tmp;
    }

    println!("{}", sum);
}

fn main() {
    problem_1();
    problem_2();
}