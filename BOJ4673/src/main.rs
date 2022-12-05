use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    for n in 1..10001 {
        let mut cur = get_next(n);
        // println!("{}", cur);
        while map.get(&cur) == None && cur <= 10000 {
            map.insert(cur, 0);
            cur = get_next(cur);
        }
    }

    for n in 1..10001 {
        match map.get(&n) {
            None => println!("{}", n),
            _ => (),
        };
    }
}

fn get_next(n: i32) -> i32 {
    let mut next = n;
    // print!("get_next({}) = ", n);
    let mut n = n;
    while n != 0 {
        next += n % 10;
        n /= 10;
    }
    // println!("{}", next);
    next
}
