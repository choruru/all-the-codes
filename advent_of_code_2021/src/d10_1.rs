#[macro_use]
extern crate maplit;
use std::io;

fn main() {
    let mut line = String::new();
    let mut ans = 0;
    let map = hashmap! {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
    };
    let score = hashmap! {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
    };
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }
        let mut s = vec![];
        for c in line.chars().into_iter() {
            if map.contains_key(&c) {
                if s.is_empty() || s.pop().unwrap() != *map.get(&c).unwrap() {
                    ans += score.get(&c).unwrap();
                    break;
                }
            } else {
                s.push(c);
            }
        }
        line.clear();
    }
    println!("{:?}", ans);
}
