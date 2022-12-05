use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n: i32 = s.trim().parse().unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let scores: Vec<i32> = s
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let max = scores.iter().max();
    let sum: i32 = scores.iter().sum();

    match max {
        Some(i) => {
            let new_avg: f64 = sum as f64 / scores.len() as f64 * 100.0 / *i as f64;
            println!("{}", new_avg);
        },
        _ => (),
    }
}
