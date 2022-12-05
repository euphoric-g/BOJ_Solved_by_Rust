use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let t: i32 = s.trim().parse().unwrap();

    for _tc in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        
        let mut score = 0;
        let mut seq = 1;

        for i in s.chars() {          
            match i as u8 {
                b'O' => {
                    score += seq;
                    seq += 1;
                },
                b'X' => {
                    seq = 1;
                },
                _ => (),
            };
        }

        println!("{}", score);
    }
}
