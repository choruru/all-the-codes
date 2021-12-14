use std::collections::HashMap;
use std::io;

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).unwrap();
    let chars = line.trim().chars().collect::<Vec<char>>();
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

    // init
    let mut pattern_cnt: HashMap<String, i64> = HashMap::new();
    for key in rules.keys() {
        pattern_cnt.insert(key.to_string(), 0);
    };
    let mut i = 0;
    while i < chars.len() - 1 {
        let key = [chars[i], chars[i + 1]].iter().collect::<String>();
        if pattern_cnt.contains_key(&key) {
            let val = pattern_cnt.get_mut(&key.clone()).unwrap();
            *val += 1;
        }
        i += 1;
    }

    let mut char_to_cnt: HashMap<char, i64> = HashMap::new();
    for c in chars.into_iter() {
        let val = char_to_cnt.entry(c).or_insert(0);
        *val += 1;
    };

    // iteration
    for _n in 0..40 {
        let mut pattern_cnt_diff: HashMap<String, i64> = HashMap::new();
        for (pattern, cnt) in pattern_cnt.clone().iter() {
            // add count for newly inserted char
            let inserted_char = rules[pattern];
            let val = char_to_cnt.entry(inserted_char).or_insert(0);
            *val += cnt;

            // remove this pattern
            pattern_cnt_diff.insert(pattern.clone(), -cnt);

            // add new pattens
            let chars = pattern.chars().collect::<Vec<char>>();
            let new_pat1 = [chars[0], inserted_char].iter().collect::<String>();
            let new_pat2 = [inserted_char, chars[1]].iter().collect::<String>();
            for new_pat in [new_pat1, new_pat2].iter() {
                if pattern_cnt.contains_key(new_pat) {
                    pattern_cnt_diff.insert(new_pat.clone(), *cnt);
                }
            };
        }

        // recalc patten_cnt
        for (pattern, cnt) in pattern_cnt_diff.into_iter() {
            let val = pattern_cnt.get_mut(&pattern).unwrap();
            *val += cnt;
        }
    };

    println!("{}", char_to_cnt.values().max().unwrap() - char_to_cnt.values().min().unwrap());
}
