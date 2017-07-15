use std::io;

fn main() {
    loop {
        println!("Enter number:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect(
            "Input failed"
        );
        let input: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("Fibonacci {}", fib(input))
    }
    
}

fn fib(num: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    let mut n = num;
    while n > 1 {
        let t = a;
        a = b;
        b += t;
        n -= 1;
    }
    b
}