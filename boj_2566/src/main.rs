use std::io;

fn main() {
    let mut arr = vec![vec![0; 9]; 9];

    for i in 0..9 {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let s: Vec<usize> = s
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        for j in 0..9 {
            arr[i][j] = s[j];
        }
    }

    let flattened_arr = arr.iter().flatten().enumerate().collect::<Vec<(usize, &usize)>>();

    let (idx, val) = flattened_arr
        .iter()
        .max_by_key(|(idx, val)| val)
        .unwrap();

    println!("{val}\n{} {}", 1+idx/9, 1+idx%9);
}
