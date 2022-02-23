// Multiples of 3 or 5
fn problem_1(ceil: u32) {
    let mut sum: u32 = 0;
    for x in (3..ceil).step_by(3) {
        sum += x;
    }
    for y in (5..ceil).step_by(5) {
        sum += if y % 15 == 0 { 0 } else { y };
    }

    println!("{}", sum);
}

// Even Fibonacci numbers
fn problem_2(ceil: u32) {
    let mut x:      u32 = 1;
    let mut y:      u32 = 2;
    let mut tmp:    u32;
    let mut sum = 0;

    while x < ceil && y < ceil {
        if y % 2 == 0 {
            sum += y;
        }
        tmp = x + y;
        x = y;
        y = tmp;
    }

    println!("{}", sum);
}

// Largest prime factor
fn problem_3(mut num: u64) {
    let mut checked_primes = Vec::new();
    let mut primes = Vec::new();
    let mut curr_prime = 2;

    while curr_prime <= num {
        if num % curr_prime == 0 {
            primes.push(curr_prime);
            num /= curr_prime;
        } else {
            let mut is_not_prime;
            checked_primes.push(curr_prime);

            while {
                is_not_prime = false;
                curr_prime += 1;
                for prime in &checked_primes {
                    if curr_prime % prime == 0 {
                        is_not_prime = true;
                        break;
                    }
                }
                is_not_prime
            } {}
        }
    }

    let max_prime = primes.iter().max();
    match max_prime {
        Some(max)   => println!("{}", max),
        None        => println!("No Solution.")
    }
}

// Largest palindrome product
fn problem_4() {
    let mut largest_palindrome = 0;
    for i in 100..1000 {
        for j in i..1000 {
            let num = i * j;
            let num_string = num.to_string();
            let reversed_string = num_string.chars().rev().collect::<String>();
            if num_string.eq(&reversed_string) && num > largest_palindrome {
                largest_palindrome = num;
            }
        }
    }

    println!("{}", largest_palindrome);
}

// Smallest multiple
fn problem_5(max_div: u32) {
    let mut multiple = max_div;
    let mut not_smallest_multiple;
    while {
        not_smallest_multiple = false;
        for div in 2..=max_div {
            if multiple % div != 0 {
                not_smallest_multiple = true;
                multiple += max_div;
                break;
            }
        }
        not_smallest_multiple
    } {}
    println!("{}", multiple);
}

// Sum square difference
fn problem_6(ceil: u64) {
    let mut sum_of_squares: u64 = 1;
    let mut square_of_sum: u64 = 1;

    for curr in 2..=ceil {
        sum_of_squares = sum_of_squares + curr * curr;
        square_of_sum += curr;
    }
    square_of_sum *= square_of_sum;
    println!("{}", square_of_sum - sum_of_squares);
}

fn main() {
    problem_1(1000);
    problem_2(4000000);
    problem_3(600851475143);
    problem_4();
    problem_5(20);
    problem_6(100);
}