use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let words: Vec<&str> = s.split_whitespace().collect();
    println!("{}", words.len());
}
