use std::io;

fn main() {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let n: i32 = s.trim().parse().unwrap();

    let mut s = String::new();
    
    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i32> = s
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let minValue: Option<&i32> = values.iter().min();

    if let Some(i) = minValue {
        print!("{} ", i);
    }

    let maxValue: Option<&i32> = values.iter().max();

    if let Some(i) = maxValue {
        println!("{}", i);
    }


}
