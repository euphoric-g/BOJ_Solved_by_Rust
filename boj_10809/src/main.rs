use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut arr = [-1; 26];

    for (idx, ch) in s.trim().chars().enumerate() {
        let arr_idx = (ch as u8 - b'a' as u8) as usize;
        let val: &mut i32 = &mut arr[arr_idx];

        if *val == -1 {
            *val = idx as i32;
        }
    }

    for i in 0..26 {
        print!("{} ", arr[i]);
    }
    println!();
}
