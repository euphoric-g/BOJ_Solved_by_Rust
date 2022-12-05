use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let s = s.trim();
    let mut count = [0; 26];

    for ch in s.to_lowercase().chars() {
        count[(ch as u8 - b'a') as usize] += 1;
    }

    let mut max_idx = 0;
    let mut frequency = -1;
    let mut cnt = 0;

    for i in 0..26 {
        if count[i] > frequency {
            max_idx = i;
            frequency = count[i];
            cnt = 1;
        } else if count[i] == frequency {
            cnt += 1;
        }
    }

    if cnt == 1 {
        println!("{}", (b'A' as u8 + max_idx as u8) as char);
    } else {
        println!("?");
    }


}
