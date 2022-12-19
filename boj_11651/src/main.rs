use std::io;
use io::Write;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n : usize = s.trim().parse().unwrap();
    let mut v = Vec::<(i32, i32)>::new();
    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let point: Vec<i32> = s
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        v.push((point[0], point[1]));
    }
    v.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    for elem in v {
        writeln!(out, "{} {}", elem.0, elem.1).unwrap();
    }
}