use std::io;
use std::cmp;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let ab: Vec<i32> = s.split_whitespace().map(|s| s.parse().unwrap()).collect();
    
    let (a, b) = (ab[0]/2, ab[1]/2);

    println!("{}", cmp::min(a, b));
}