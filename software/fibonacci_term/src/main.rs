use std::io;

fn main() {
    let mut n = String::new();

    println!("Please enter a number greater than 0: ");

    io::stdin().read_line(&mut n).expect("failed to read input");

    let n: u32 = n.trim().parse().expect("Please type a number!");

    println!("The {}° number of the fibonacci sequence is: {}", n, fibonacci(n));
}

fn fibonacci(n: u32) -> u32 {
    if n == 1 {
        return 0;
    } else if n == 2 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}