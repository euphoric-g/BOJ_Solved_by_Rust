use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut sum: i32 = 0;

    for n in s.trim().chars() {
        sum += (n as u8 - 48) as i32;
    }

    println!("{}", sum);
}
