use std::io;

fn main() {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let t: i32 = s
        .trim()
        .parse()
        .unwrap();

    let mut tc = 1;

    while tc <= t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        
        let values: Vec<i32> = s
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        println!("Case #{}: {} + {} = {}", tc, values[0], values[1], values[0] + values[1]);

        tc += 1;
    }
}
