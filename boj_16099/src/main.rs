use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let t: i32 = s.trim().parse().unwrap();

    for _ in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let v: Vec<i64> = s.split_whitespace().map(|s| s.parse().unwrap()).collect();

        let telecom_paris_tech: i64 = v[0] * v[1];
        let eurecom: i64 = v[2] * v[3];

        if eurecom > telecom_paris_tech {
            println!("Eurecom");
        } else if eurecom < telecom_paris_tech {
            println!("TelecomParisTech");
        } else {
            println!("Tie");
        }
    }
}