use std::io;

fn main() {
    let mut v = Vec::<usize>::new();

    let mut sum = 0;

    for _ in 0..5 {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let n = s.trim().parse().unwrap();

        sum += n;
        v.push(n);
        
    }

    v.sort();

    println!("{}\n{}", sum / 5, v[2]);
}
