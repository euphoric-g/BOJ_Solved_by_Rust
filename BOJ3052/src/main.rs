use std::io;
use std::collections::HashMap;

fn main() {

    let mut hash_map = HashMap::new();
    let mut cnt = 10;

    while cnt > 0 {
        cnt -= 1;
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let key: i32 = s.trim().parse().unwrap();
        hash_map.entry(key % 42).or_insert(0);
    }

    println!("{}", hash_map.keys().len());
}
