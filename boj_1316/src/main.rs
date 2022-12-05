use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let t: i32 = s.trim().parse().unwrap();

    let mut group_word = 0;

    for _ in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let s = s.trim();

        let s2 = s.clone();

        let mut s = s.chars().collect::<Vec<char>>();
        s.dedup();

        let mut s2 = s2.chars().collect::<Vec<char>>();
        s2.sort_unstable();
        s2.dedup();

        if s.len() == s2.len() {
            group_word += 1;
        }
    }

    println!("{group_word}");
}
