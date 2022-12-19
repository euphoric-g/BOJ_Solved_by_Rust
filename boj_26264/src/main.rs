use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n: usize = s.trim().parse().unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut bigdatas = s.as_str().matches("bigdata").count();
    let mut securities = s.as_str().matches("security").count();
    let common = bigdatas + securities - n;
    bigdatas -= common;
    securities -= common;

    if bigdatas > securities {
        println!("bigdata?");
    } else if bigdatas < securities {
        println!("security!");
    } else {
        println!("bigdata? security!");
    }
}