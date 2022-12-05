use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    for c in s.trim().bytes() {
        println!("{}", c);
    }
}
