use std::io;
use io::Write;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse().unwrap();
    let mut v = vec![0; 10001];

    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let num: usize = s.trim().parse().unwrap();
        v[num] += 1;
    }

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    
    for i in 0..10001 {
        for _ in 0..v[i] {
            writeln!(out, "{i}").unwrap();
        }
    }
}
