use std::io;
use std::collections::HashMap;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let _nm: Vec<i32> = s.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut map = HashMap::new();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let vec_a: Vec<i32> = s.split_whitespace().map(|s| s.parse().unwrap()).collect();
    
    for elem in &vec_a {
        map.entry(elem).or_insert(0);
    }

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let vec_b: Vec<i32> = s.split_whitespace().map(|s| s.parse().unwrap()).collect();

    for elem in &vec_b {
        map.entry(elem).or_insert(0);
    }

    let common = vec_a.len() + vec_b.len() - map.keys().len();
    println!("{}", map.keys().len() - common);
}
