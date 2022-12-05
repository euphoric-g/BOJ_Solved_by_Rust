use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let vec_char = s.trim().chars().collect::<Vec<char>>();
    let iter = vec_char.chunks(10);

    let strings = iter
        .map(|v| v.iter().collect::<String>())
        .collect::<Vec<String>>();

    for string in strings {
        println!("{string}");
    }
}