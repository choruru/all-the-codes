use std::collections::HashMap;
use std::io;

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).unwrap();
    let mut chars = line.trim().chars().collect::<Vec<char>>();
    line.clear();

    let mut rules = HashMap::new();
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if line == "\n" {
            continue;
        }
        if n == 0 {
            break;
        }
        let l = line.trim().split(" -> ").collect::<Vec<&str>>();
        rules.insert(l[0].to_string(), l[1].chars().next().unwrap());
        line.clear();
    }

    for _n in 0..40 {
        let mut i = 0;
        while i < chars.len() - 1 {
            let key = [chars[i], chars[i + 1]].iter().collect::<String>();
            if rules.contains_key(&key) {
                chars.insert(i + 1, rules[&key]);
                i += 1;
            }
            i += 1;
        }
    };

    let mut char_to_cnt = HashMap::new();
    for c in chars.into_iter() {
        let val = char_to_cnt.entry(c).or_insert(0);
        *val += 1;
    }

    println!("{}", char_to_cnt.values().max().unwrap() - char_to_cnt.values().min().unwrap());
}
