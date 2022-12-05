use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let ans: i32 = s
        .split_whitespace()
        .map(|s| s.chars().rev().collect::<String>())
        .map(|s| s.parse().unwrap())
        .max()
        .unwrap();

    println!("{}", ans);
}
