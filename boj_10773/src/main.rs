use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let k: i32 = s.trim().parse().unwrap();

    let mut v = Vec::<i32>::new();

    for _ in 0..k {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let num: i32 = s.trim().parse().unwrap();

        if num == 0 {
            v.pop();
        } else {
            v.push(num);
        }
    }

    println!("{}", v.iter().sum::<i32>());
}