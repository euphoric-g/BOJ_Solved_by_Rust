use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let a: i32 = s.trim().parse().unwrap();
    
    let mut i = 1;
    let mut sum = 1;

    loop {
        if sum >= a {
            break;
        }
        sum += i * 6;
        i += 1;
    }
    println!("{}", i);
}
