use std::io::stdin;

fn main() {
    println!("Enter n");

    let mut n: String = String::new();

    stdin().read_line(&mut n).expect("Could not read line");

    let n: u32 = n.trim().parse().expect("Please enter a number");

    let x = fib(n);

    println!("{}", x)
}

fn fib(n: u32) -> u32 {
    if n <= 1 { n } else { fib(n - 1) + fib(n - 2) }
}
