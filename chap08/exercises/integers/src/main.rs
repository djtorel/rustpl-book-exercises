extern crate rand;

use rand::Rng;
use std::collections::HashMap;

fn main() {
    // Generate arbitrary amount of random numbers to compute
    let mut numbers = Vec::new();
    let amount = 30000;
    let min = 0;
    let max = 100000;
    println!("Generating {} integers between {} and {}", amount, min, max);

    for _ in 0..amount {
        let num: u32 = rand::thread_rng().gen_range(min, max);
        numbers.push(num);
    }

    // Find average by calculating sum and divide by amount
    let mut sum: u32 = 0;
    for &number in &numbers {
        sum += number;
    }
    let avg = sum / amount;
    println!("Average: {} / {} = {}", sum, amount, avg);

    // Find median by sorting vector then find middle position
    numbers.sort();
    let median = numbers[(amount/2) as usize];
    println!("median is: {}", median);

    // Find mode? Value with most entries... Suggested using hashmap
    let mut number_hash = HashMap::new();
    for &number in &numbers {
        let count = number_hash.entry(number).or_insert(0);
        *count += 1;
    }

    let mut largest_val = 0;
    let mut mode = 0;
    for (key, val) in number_hash.iter() {
        if *val > largest_val {
            largest_val = *val;
            mode = *key;
        }
    }
    println!("mode: {}", mode);
    // println!("{:?}", numbers);
}
