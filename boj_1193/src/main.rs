use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let a: i32 = s.trim().parse().unwrap();

    let mut i = 0;
    let mut sum = 1;
    
    loop {
        sum += i;
        i += 1;
        if sum > a {
            sum -= i-1;
            break;
        }
    }
    if i%2 == 0 {
        println!("{}/{}", i-(a%sum+1), a%sum+1);
    } else {
        println!("{}/{}", a%sum+1, i-(a%sum+1));
    }
}
