#[macro_use]
extern crate maplit;
use std::io;

fn main() {
    let mut line = String::new();
    let mut ans = vec![];
    let map = hashmap! {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
    };
    let score = hashmap! {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
    };
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }
        let mut s = vec![];
        let mut valid = true;
        for c in line.trim().chars().into_iter() {
            if map.contains_key(&c) {
                if s.is_empty() || s.pop().unwrap() != *map.get(&c).unwrap() {
                    valid = false;
                    break;
                }
            } else {
                s.push(c);
            }
        }
        if valid {
            let mut x: i64 = 0;
            while let Some(top) = s.pop() {
                x = 5 * x + score[&top];
            }
            ans.push(x);
        }
        line.clear();
    }
    ans.sort();
    println!("{:?}", ans[ans.len() / 2]);
}
