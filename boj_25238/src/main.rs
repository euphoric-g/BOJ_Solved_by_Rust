use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let ab: Vec<i32> = s.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let def: f64 = (ab[0] as f64) * ((100 - ab[1]) as f64) / 100.0;

    println!("{}", if def >= 100.0 { 0 } else { 1 });
}