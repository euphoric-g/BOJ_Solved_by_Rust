use std::io;

fn main() {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let abc: Vec<i64> = s
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let (a, b, c) = (abc[0], abc[1], abc[2]);

    if b >= c {
        println!("-1");
    } else {
        println!("{}", a/(c-b)+1);
    }
}
