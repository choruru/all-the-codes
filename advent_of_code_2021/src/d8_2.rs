#[macro_use]
extern crate maplit;
use std::collections::{HashMap, HashSet};
use std::io;
use std::iter::FromIterator;

fn main() {
    let mut line = String::new();
    let mut ans = 0;
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }
        let parts = line.trim().split(" | ").collect::<Vec<&str>>();
        let inputs = parts[0]
            .split(" ")
            .map(|s| -> Vec<char> {
                let mut chars = s.chars().collect::<Vec<char>>();
                chars.sort();
                chars
            })
            .collect::<Vec<Vec<char>>>();
        let outputs = parts[1]
            .split(" ")
            .map(|s| -> Vec<char> {
                let mut chars = s.chars().collect::<Vec<char>>();
                chars.sort();
                chars
            })
            .collect::<Vec<Vec<char>>>();

        // number of segments -> list of number chars
        let mut segnum_to_chars = HashMap::new();
        for c in inputs.iter() {
            let segnum = c.len();
            if !segnum_to_chars.contains_key(&segnum) {
                segnum_to_chars.insert(segnum, vec![]);
            }
            segnum_to_chars.get_mut(&segnum).unwrap().push(c);
        }

        let mut zero = &vec![];
        let one;
        let mut two = &vec![];
        let mut three = &vec![];
        let four;
        let mut five = &vec![];
        let mut six = &vec![];
        let seven;
        let eight;
        let mut nine = &vec![];

        one = segnum_to_chars.get(&2).unwrap()[0];
        seven = segnum_to_chars.get(&3).unwrap()[0];
        four = segnum_to_chars.get(&4).unwrap()[0];
        eight = segnum_to_chars.get(&7).unwrap()[0];

        // numbers with 5 segments
        let segs_of_1: HashSet<&char> = HashSet::from_iter(one);
        let segs_of_4 = HashSet::from_iter(four);
        for &c in segnum_to_chars.get(&5).unwrap().into_iter() {
            let segs = HashSet::from_iter(c.into_iter());
            if segs.difference(&segs_of_1).count() == 3 {
                three = c;
            } else if segs.difference(&segs_of_4).count() == 3 {
                two = c;
            } else {
                five = c;
            }
        }

        // numbers with 6 segments
        let segs_of_5 = HashSet::from_iter(five);
        for &c in segnum_to_chars.get(&6).unwrap().into_iter() {
            let segs = HashSet::from_iter(c.into_iter());
            if segs.difference(&segs_of_4).count() == 2 {
                nine = c;
            } else if segs.difference(&segs_of_5).count() == 1 {
                six = c;
            } else {
                zero = c;
            }
        }

        let chars_to_num = hashmap! {
            zero => 0,
            one => 1,
            two => 2,
            three => 3,
            four => 4,
            five => 5,
            six => 6,
            seven => 7,
            eight => 8,
            nine => 9,
        };

        let mut n = 0;
        for c in outputs.into_iter() {
            n *= 10;
            n += chars_to_num.get(&c).unwrap();
        }
        ans += n;
        line.clear();
    }
    println!("{:?}", ans);
}
