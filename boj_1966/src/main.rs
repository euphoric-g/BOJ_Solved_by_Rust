use std::io;
use std::Collections::VecDeque;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let t: usize = s.trim().parse().unwrap();

    for _ in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let nm: Vec<usize> = s
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let queue: VecDeque<usize> = s
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let out = Vec::<usize>::new();

        while queue.len() > 0 {
            let front = queue.pop_front();
            if *queue.iter().max() > front {
                queue.push_back(front);
            } else {
                out.push_back(front);
            }
        }
    }
}