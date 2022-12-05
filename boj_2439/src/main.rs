use std::io;

fn main() {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let n: i32 = s.trim().parse().unwrap();

    for i in (0..n).rev() {
        for j in 0..i {
            print!(" ");
        }
        for j in 0..n-i {
            print!("*");
        }
        println!();
    }
}
