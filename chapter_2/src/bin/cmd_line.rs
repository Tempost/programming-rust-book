use std::env;
use std::str::FromStr;

use chapter_2::gcd;

fn main() {
    let numbers: Vec<u64> = env::args()
        .skip(1)
        .map(|arg| u64::from_str(&arg).expect("error parsing argument"))
        .collect();

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest divisor of {:?} is {}", numbers, d);
}
