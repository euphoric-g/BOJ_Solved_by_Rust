use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n: u32 = s.trim().parse().unwrap();
    let base: i32 = 2;

    println!("{}", base.pow(n));
}