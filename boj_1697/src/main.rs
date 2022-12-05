use std::io;
use std::collections::VecDeque;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let params: Vec<usize> = s
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let (n, k) = (params[0], params[1]);

    let mut cost = vec![-1; 100001];
    let mut q: VecDeque<i32> = VecDeque::new();

    q.push_back(n as i32);
    cost[n] = 0;

    while let Some(now) = q.pop_front() {
        println!("{}", now);
        if now as usize == k {
            break;
        }

        if now - 1 >= 0 && cost[(now-1) as usize] != -1 {
            cost[(now-1) as usize] = cost[now as usize] + 1;
            q.push_back(now-1);
        }

        if now + 1 <= 100000 && cost[(now+1) as usize] != -1 {
            cost[(now+1) as usize] = cost[now as usize] + 1;
            q.push_back(now+1);
        }

        if now * 2 <= 100000 && cost[(now*2) as usize] != -1 {
            cost[(now*2) as usize] = cost[now as usize] + 1;
            q.push_back(now*2);
        }
    }

    println!("{}", cost[k]);
}
