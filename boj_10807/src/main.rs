use std::io;

fn main() {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let n: i32 = s.trim().parse().unwrap();

    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i32> = s
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let v: i32 = s.trim().parse().unwrap();

    println!("{}", values.iter().filter(|&n| *n == v).count());
}
