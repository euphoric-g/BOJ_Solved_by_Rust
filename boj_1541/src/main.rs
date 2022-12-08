use std::io;

fn add_all(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut numbers: Vec<i32> = s
        .split(&['+', '-', '\n'][..])
        .map(|s| s.parse())
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap())
        .collect();
    let operators: Vec<&str> = s.split(char::is_numeric).filter(|&x| x == "+" || x == "-").collect();

    match operators.iter().position(|&s| s == "-") {
        Some(pos) => {
            let numbers2 = numbers.split_off(pos+1);
            println!("{}", add_all(&numbers[..]) - add_all(&numbers2[..]));
        },
        None => {
            println!("{}", add_all(&numbers[..]));
        }
    }
}