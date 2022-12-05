use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let nk: Vec<usize> = s
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let mut vec: Vec<usize> = s
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    vec.sort();

    println!("{}", vec.iter().nth(nk[0]-nk[1]).unwrap());
}
