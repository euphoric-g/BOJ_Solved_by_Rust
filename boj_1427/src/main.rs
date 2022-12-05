use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let mut v: Vec<char> = s.trim().chars().collect();
    v.sort();
    let mut v: Vec<char> = v.into_iter().rev().collect();
    for ch in v {
        print!("{ch}");
    }
    println!();
}
