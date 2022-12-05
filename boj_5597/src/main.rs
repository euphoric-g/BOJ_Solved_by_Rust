use std::io;
use std::collections::HashMap;

fn main() {
    let mut codes = HashMap::new();
    let mut cnt = 28;

    while cnt > 0 {
        cnt -= 1;
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let num: i32 = s.trim().parse().unwrap();
        codes.insert(num, 1);
    }

    for i in 1..31 {
        let submitted = codes.get(&i);
        match submitted {
            None => println!("{}", i),
            _ => (),
        }
    }
}
