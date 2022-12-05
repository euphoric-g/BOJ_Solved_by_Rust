use std::io;

fn main() {
    loop {
        let mut s = String::new();

        let byte_read = io::stdin().read_line(&mut s).unwrap();

        if byte_read == 0 {
            break;
        }
        
        let values: Vec<i32> = s
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        println!("{}", values[0] + values[1]);
    }
}
