use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let c: i32 = s.trim().parse().unwrap();

    for _ in 0..c {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let scores: Vec<i32> = s
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
            
        let sum: i32 = scores.iter().sum::<i32>() - scores[0];
        let avg: f64 = sum as f64 / scores[0] as f64;

        let ans = &scores[1..].iter().filter(|&&s| s as f64 > avg).count();

        println!("{:.3}%", *ans as f64 / scores[0] as f64 * 100.0);
    }
}