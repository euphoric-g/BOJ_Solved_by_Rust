use std::io;

fn hansu(num: usize) -> bool {
    let num: String = num.to_string();
    match num.len() {
        0..=2 => true,
        3 => {
            let v: Vec<i32> = num.chars().map(|s| s.to_digit(10).unwrap() as i32).collect();
            v[1] - v[0] == v[2] - v[1]
        },
        _ => false
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n: usize = s.trim().parse().unwrap();

    let mut cnt = 0;

    for num in 1..=n {
        if hansu(num) { cnt += 1; }
    }

    println!("{cnt}");
}