use std::io;

fn main() {
    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        let v: Vec<&str> = s.split_whitespace().collect();
        let (name, age, weight): (&str, i32, i32) = (v[0], v[1].parse().unwrap(), v[2].parse().unwrap());

        if name == "#" {
            break;
        }

        if age > 17 || weight >= 80 {
            println!("{name} Senior");
        } else {
            println!("{name} Junior");
        }
    }
}