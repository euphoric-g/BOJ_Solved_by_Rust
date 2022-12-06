use std::io;

fn main() {
    loop {
        let mut s = String::new();
        match io::stdin().read_line(&mut s) {
            Ok(0) => {
                break;
            }
            _ => {
                print!("{s}");
            }
        }
    }
}
