use std::io;

fn main() {
    let special_cases = vec!["c=", "c-", "dz=", "d-", "lj", "nj", "s=", "z="];

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim();

    let mut count = s.len();

    for sc in special_cases {
        let matches = s.matches(sc).count();
        count -= matches;
    }

    println!("{}", count);
}
