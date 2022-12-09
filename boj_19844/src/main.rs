use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut v: Vec<&str> = s.split(&[' ', '-', '\n'][..]).filter(|&x| x.len() > 0).collect();
    let has_apostrophe = v.iter().filter(|&x| x.find(|x| x == '`') != None);
    
    let mut words = v.len();

    loop {
        match has_apostrophe.next() {
            Some(w) => {
                let chars = w.chars();
                if let Some(pos) = chars.find(|x| x == '`') {
                    if &['a', 'e', 'i', 'o', 'u', 'h'].find(|x| x == chars[pos+1]) != None &&
                    
                }
            },
            None => {
                break;
            }
        }
    }

    println!("{words}");
}