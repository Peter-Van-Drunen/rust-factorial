use std::io::{stdin};

fn factorial(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

fn main() {
    println!("Enter a number for factorial:");

    let mut input = String::new();

    stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");

    let n = input.trim().parse::<i32>().ok().expect("Couldn't uwrap");

    println!("factorial was {}", factorial(n));
}
