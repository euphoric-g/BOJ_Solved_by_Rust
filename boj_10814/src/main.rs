use std::io;
use io::Write;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n: usize = s.trim().parse().unwrap();

    let mut members = Vec::<(usize, String)>::new();

    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let data: Vec<&str> = s
            .split_whitespace()
            .collect();

        members.push((data[0].parse().unwrap(), data[1].to_string()));
    }

    members.sort_by(|a, b| a.0.cmp(&b.0));

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for member in members {
        writeln!(out, "{} {}", member.0, member.1).unwrap();
    }
}
