use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let nx: Vec<i32> = s.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let values: Vec<i32> = s.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let values_filtered: Vec<&i32> = values.iter().filter(|&n| *n < nx[1]).collect();

    for i in values_filtered {
        print!("{} ", i);
    }
}
