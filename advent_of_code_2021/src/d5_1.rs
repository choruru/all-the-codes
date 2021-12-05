use std::cmp;
use std::collections::HashSet;
use std::io;

fn main() {
    let mut line = String::new();
    let mut appeared = HashSet::new();
    let mut dup = HashSet::new();
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }
        let mut iter = line.trim().split(" -> ");
        let p1 = iter
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();
        let p2 = iter
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();

        if p1[0] == p2[0] {
            for y in cmp::min(p1[1], p2[1])..=cmp::max(p1[1], p2[1]) {
                let key = format! {"{}:{}", p1[0], y};
                if appeared.contains(&key) {
                    dup.insert(key.to_string());
                }
                appeared.insert(key.to_string());
            }
        } else if p1[1] == p2[1] {
            for x in cmp::min(p1[0], p2[0])..=cmp::max(p1[0], p2[0]) {
                let key = format! {"{}:{}", x, p1[1]};
                if appeared.contains(&key) {
                    dup.insert(key.to_string());
                }
                appeared.insert(key.to_string());
            }
        }
        line.clear();
    }
    println!("{}", dup.len());
}
