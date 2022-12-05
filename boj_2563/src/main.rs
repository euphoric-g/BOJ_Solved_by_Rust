use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let t = s.trim().parse().unwrap();

    let mut arr = vec![vec![0; 100]; 100];

    for _ in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let xy: Vec<usize> = s
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (x, y) = (xy[0], xy[1]);

        for xx in x..x+10 {
            for yy in y..y+10 {
                arr[xx][yy] = 1;
            }
        }
    }

    println!("{}", arr.iter().flatten().filter(|&x| *x == 1).count());
}
