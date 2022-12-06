use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let nm: Vec<i32> = s
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for _ in 0..nm[0] {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        println!("{}", s.trim().chars().rev().collect::<String>());
    }
}