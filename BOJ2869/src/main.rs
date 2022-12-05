use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let params: Vec<i32> = s
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let (a, b, c) = (params[0], params[1], params[2]);

    if c-(a-b)*((c-b)/(a-b)) > b {
        println!("{}", (c-b)/(a-b)+1);
    } else {
        println!("{}", (c-b)/(a-b));
    
}
