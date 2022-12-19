use std::io;
use io::Write;
use std::collections::VecDeque;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut q = VecDeque::<i32>::new();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n : i32 = s.trim().parse().unwrap();

    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        let args: Vec<&str> = s.split_whitespace().collect();
        
        if args.len() == 1 {
            if args[0] == "top" {
                if q.len() == 0 {
                    writeln!(out, "-1").unwrap();
                } else {
                    let back = q.pop_back().unwrap();
                    q.push_back(back);
                    writeln!(out, "{back}").unwrap();
                }
            } else if args[0] == "size" {
                writeln!(out, "{}", q.len()).unwrap();
            } else if args[0] == "empty" {
                writeln!(out, "{}", if q.len() == 0 { 1 } else { 0 }).unwrap();
            } else if args[0] == "pop" {
                if q.len() == 0 {
                    writeln!(out, "-1").unwrap();
                } else {
                    writeln!(out, "{}", q.pop_back().unwrap()).unwrap();
                }
            }
        } else if args.len() == 2 {
            let num: i32 = args[1].parse().unwrap();
            q.push_back(num);
        }
    }
}