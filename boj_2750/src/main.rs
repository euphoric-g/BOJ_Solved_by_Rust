use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n: usize = s.trim().parse().unwrap();
    
    let mut v = Vec::<isize>::new();

    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        v.push(s.trim().parse().unwrap());
    }

    v.sort();

    for sorted_element in &v {
        println!("{sorted_element}");
    }
}
