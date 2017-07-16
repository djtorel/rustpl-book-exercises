fn main() {
    let mut v: Vec<u64> = Vec::new();
    let x = 200;
    for i in 1..x+1 {
        for b in 1..x+1 {
            for c in 1..x+1 {
                v.push(i*b*c);
            }
        }
    }
    let mut sum: u64 = 0;
    for &num in &v {
        sum += num;
    }

    println!("sum: {}", sum);
    println!("length of v: {}", v.len())
}
