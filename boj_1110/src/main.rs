use std::io;

fn main() {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let n: i32 = s.trim().parse().unwrap();

    let mut cur = next(n);
    let mut cycles = 1;

    while n != cur {
        cur = next(cur);
        cycles += 1;
    }

    println!("{}", cycles);
}

fn next(mut num: i32) -> i32 {
    num = 10 * (num % 10) + (num / 10 + num % 10) % 10;
    num
}
