use std::io;

fn main() {
    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        let values: Vec<i32> = s
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if values[0] == 0 && values[1] == 0 {
            break;
        } else {
            println!("{}", values[0] + values[1]);
        }
    }
}
