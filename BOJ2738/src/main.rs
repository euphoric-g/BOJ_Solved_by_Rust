use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let nm: Vec<usize> = s
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let (n, m) = (nm[0], nm[1]);
    let mut vec_a = vec![vec![0; m]; n];
    for i in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let s: Vec<usize> = s
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        for j in 0..m {
            vec_a[i][j] = s[j];
        }
    }
    for i in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let s: Vec<usize> = s
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        for j in 0..m {
            print!("{} ", vec_a[i][j] + s[j]);
        }
        println!();
    }
}
