use std::io;

fn main() {

    let mut maxValue = -1;
    let mut maxIndex = -1;

    for i in 1..10 {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let num: i32 = s.trim().parse().unwrap();
        if num > maxValue {
            maxValue = num;
            maxIndex = i;
        }
    }
    println!("{}\n{}", maxValue, maxIndex);
}
