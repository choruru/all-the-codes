use std::cmp::min;
use std::io;

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).unwrap();
    let mut p1: u64 = line.trim().split(": ").last().unwrap().parse().unwrap();
    p1 -= 1;
    line.clear();

    io::stdin().read_line(&mut line).unwrap();
    let mut p2: u64 = line.trim().split(": ").last().unwrap().parse().unwrap();
    p2 -= 1;

    let mut cnt = 0;
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut die = 0;
    loop {
        cnt += 1;
        if cnt % 2 == 1 {
            p1 = (p1 + (3 * (die + 1) + 3)) % 10;
            p1_score += p1 + 1;
            if p1_score >= 1000 {
                break;
            }
        } else {
            p2 = (p2 + (3 * (die + 1) + 3)) % 10;
            p2_score += p2 + 1;
            if p2_score >= 1000 {
                break;
            }
        }
        die = die + 3 % 1000;
    }

    println!("{}", 3 * cnt * min(p1_score, p2_score));
}
