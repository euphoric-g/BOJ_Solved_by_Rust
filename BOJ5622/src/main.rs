use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut sum = 0;

    for ch in s.trim().chars() {
        sum += cost(ch as u8);    
    }
    
    println!("{}", sum);
}

fn cost(ch: u8) -> i32 {
    match ch {
        b'A'..=b'C' => 3,
        b'D'..=b'F' => 4,
        b'G'..=b'I' => 5,
        b'J'..=b'L' => 6,
        b'M'..=b'O' => 7,
        b'P'..=b'S' => 8,
        b'T'..=b'V' => 9,
        b'W'..=b'Z' => 10,
        _ => -1,
    }
}
