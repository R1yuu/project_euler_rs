// Multiples of 3 or 5
fn problem_1(ceil: u32) -> u32 {
    
    let seg = |mult| -> u32 {(mult + (ceil - ceil % mult)) * (ceil / mult) / 2};

    seg(3) + seg(5) - seg(15)
}

/*
    1   2   3   5   8   13  21  34  55 
        E           E           E

    x   y  x+y x+2y 2x+3y
*/


// Even Fibonacci numbers
fn problem_2(ceil: u32) -> u32 {
    let mut x:      u32 = 1;
    let mut y:      u32 = 2;
    let mut tmp:    u32;
    let mut sum = 0;

    while y < ceil {
        sum += y;
        tmp = x + 2 * y;
        y = tmp + x + y;
        x = tmp;
    }

    /*
            | x | y | sum
        1   | 1 | 2 | 2
        2   | 5 | 8 | 10
        3   | 21| 34| 44

    */

    sum
}

// Largest prime factor
fn problem_3(mut num: u64) -> u64 {

    let mut factor: u64 = 2;

    while factor * factor <= num {
        while num % factor == 0 && num != factor {
            num /= factor;
        }

        factor += 1;
    }

    num

    /*
    let mut new_num: u64 = num;
    let mut largest_fact: u64 = 0;
    let mut cnt: u64 = 2;

    while cnt * cnt <= new_num {
        if new_num % cnt == 0 {
            new_num /= cnt;
            largest_fact = cnt;
        } else {
            cnt += 1;
        }
    }

    new_num * (new_num > largest_fact) as u64 + largest_fact * (new_num > largest_fact) as u64
    */
}

// Largest palindrome product
fn problem_4() -> i32 {

    let mut res = 0;
    for i in (100..=990).rev().step_by(11) {
        for j in (100..=999).rev() {
            let prod = i * j;

            if prod > res && {
                let mut reversed = 0;
                let mut number = i;

                while number > 0 {
                    reversed = reversed * 10 + number % 10;
                    number /= 10;
                }
                reversed == i || i == number / 10
            } {
                res = prod;
                break;
            } else if res > prod {
                break;
            }
        }
    }
    res


    /*
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

    largest_palindrome
    */
}

// Smallest multiple
fn problem_5(max_div: i32) -> i32 {
    const PRIMES: [i32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];
    let mut powers: [i32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

    for i in 2..max_div + 1 {
        let mut j = i;
        let mut p;
        for k in 0..8 {
            p = 0;
            while j > 0 && j % PRIMES[k] == 0 {
                p += 1;
                j /= PRIMES[k];
            }

            if p > powers[k] { powers[k] = p; }
        }
    }

    let mut res: i32 = 1;
    for k in 0..8 {
        let mut tmp = powers[k];
        while {
            powers[k] -= 1;
            tmp > 0
        } {
            res *= PRIMES[k];
            tmp = powers[k];
        }
    }
    res

    /*
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

    multiple
    */
}

// Sum square difference
fn problem_6(ceil: u64) -> u64 {

    let sum: u64;
    let squared: u64;

    sum = (ceil * (ceil + 1)) >> 1;
    squared = (ceil * (ceil + 1) * (2 * ceil + 1)) / 6;

    sum * sum - squared

    /*
    let mut sum_of_squares: u64 = 1;
    let mut square_of_sum: u64 = 1;

    for curr in 2..=ceil {
        sum_of_squares += curr * curr;
        square_of_sum += curr;
    }
    square_of_sum *= square_of_sum;
    square_of_sum - sum_of_squares
    */
}

fn main() {
    use std::time::Instant;

    let start = Instant::now();
    let res = problem_1(999);
    let duration = start.elapsed();
    println!("Problem 1: {}", res);
    println!("Time elapsed in problem_1() is: {:?}", duration);

    let start = Instant::now();
    let res = problem_2(4000000);
    let duration = start.elapsed();
    println!("Problem 2: {}", res);
    println!("Time elapsed in problem_2() is: {:?}", duration);

    let start = Instant::now();
    let res = problem_3(600851475143);
    let duration = start.elapsed();
    println!("Problem 3: {}", res);
    println!("Time elapsed in problem_3() is: {:?}", duration);

    let start = Instant::now();
    let res = problem_4();
    let duration = start.elapsed();
    println!("Problem 4: {}", res);
    println!("Time elapsed in problem_4() is: {:?}", duration);

    let start = Instant::now();
    let res = problem_5(20);
    let duration = start.elapsed();
    println!("Problem 5: {}", res);
    println!("Time elapsed in problem_5() is: {:?}", duration);

    let start = Instant::now();
    let res = problem_6(100);
    let duration = start.elapsed();
    println!("Problem 6: {}", res);
    println!("Time elapsed in problem_6() is: {:?}", duration);
}