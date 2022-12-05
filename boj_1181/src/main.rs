use std::io;
use io::Write;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n: i32 = s.trim().parse().unwrap();

    let mut dict = Vec::<String>::new();

    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        dict.push(s.trim().to_string());
    }

    dict.sort_by(|a, b| if a.len() != b.len() { a.len().cmp(&b.len()) }  else { a.cmp(b) } );
    dict.dedup();

    for word in dict {
        writeln!(out, "{word}").unwrap();
    }
}
