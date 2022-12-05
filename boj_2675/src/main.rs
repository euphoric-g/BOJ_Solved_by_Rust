use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let t: i32 = s.trim().parse().unwrap();

    for tc in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let values: Vec<&str> = s.split_whitespace().collect();
        let r: i32 = values[0].parse().unwrap();
        let s = values[1];
        for ch in s.chars() {
            for rep in 0..r {
                print!("{}", ch);
            }
        }
        println!();
    }
}
