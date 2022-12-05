use std::io;

fn main() {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let n: i32 = s.trim().parse().unwrap();
    
    for mut i in 1..n+1 {
        while i != 0 {
            i -= 1;
            print!("*");
        }
        println!();
    }
}
