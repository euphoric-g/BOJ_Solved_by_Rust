use std::io;
use itertools::Itertools;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let nm: Vec<i32> = s.split_whitespace().map(|s| s.parse().unwrap()).collect();
    
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let cards: Vec<i32> = s.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let mut max_sum = -1;

    for selected in (0..nm[0]).permutations(3) {
        let mut sum = 0;
        for s in selected {
            sum += cards[s as usize];
        }
        if max_sum < sum && sum <= nm[1] {
            max_sum = sum;
        }
    }

    println!("{max_sum}");
}