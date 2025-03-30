fn fibonacci(n: u32) -> u64 {
    if n <= 2 {
        return n.into();
    }
    let mut prevprev = 1;
    let mut prev = 1;
    let mut current = 0;
    for _ in 3..=n {
        current = prevprev + prev;
        prevprev = prev;
        prev = current;
    }
    current
}

use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a number to calculate its Fibonacci value:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: u32 = input.trim().parse().expect("Please enter a valid number");
    let result = fibonacci(n);
    println!("Fibonacci of {} is {}", n, result);
}
