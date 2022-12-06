use std::io;
use io::Write;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let t: i32 = s.trim().parse().unwrap();
    for _ in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        match s.trim().len() {
            6..=9 => {
                writeln!(out, "yes").unwrap();
            },
            _ => {
                writeln!(out, "no").unwrap();
            }
        }
    }
}