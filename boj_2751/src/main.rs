use std::io;
use io::Write;

fn main() {
    let mut v = Vec::<isize>::new();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let n: usize = s.trim().parse().unwrap();

    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let num: isize = s.trim().parse().unwrap();
        v.push(num);
    }

    v.sort();

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for num in v {
        writeln!(out, "{}", num).unwrap();
    }
}
