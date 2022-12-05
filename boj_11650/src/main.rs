use std::io;
use io::Write;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n: usize = s.trim().parse().unwrap();

    let mut v = Vec::<(i32, i32)>::new();

    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let xy: Vec<i32> = s
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        v.push((xy[0], xy[1]));
    }
    
    v.sort_by(|a, b| a.cmp(b));

    for coord in v {
        writeln!(out, "{} {}", coord.0, coord.1).unwrap();
    }
}
