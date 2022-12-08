use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let t: i32 = s.trim().parse().unwrap();

    for _ in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let mut v = Vec::<char>::new();

        let mut valid = true;

        for ch in s.trim().chars() {
            if ch == b'(' as char {
                v.push(b'(' as char);
            } else if ch == b')' as char {
                match v.pop() {
                    Some(back) => {
                        if back != b'(' as char {
                            valid = false;
                        }
                    },
                    _ => {
                        valid = false;
                    }
                }
            }
        }

        if v.len() != 0 {
            valid = false;
        }

        println!("{}", if valid { "YES" } else { "NO" });

    }
}