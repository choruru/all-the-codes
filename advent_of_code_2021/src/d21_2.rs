use cached::proc_macro::cached;
use itertools::iproduct;
use std::cmp::max;
use std::io;

#[cached]
fn dfs(p1: u64, p1_score: u64, p2: u64, p2_score: u64, p1_turn: bool) -> (u64, u64) {
    if p1_score >= 21 {
        return (1, 0);
    };
    if p2_score >= 21 {
        return (0, 1);
    };

    let mut p1_win = 0;
    let mut p2_win = 0;
    if p1_turn {
        for (i, j, k) in iproduct!(1..4, 1..4, 1..4) {
            let p = (p1 + (i + j + k)) % 10;
            let (x, y) = dfs(p, p1_score + p + 1, p2, p2_score, !p1_turn);
            p1_win += x;
            p2_win += y;
        }
    } else {
        for (i, j, k) in iproduct!(1..4, 1..4, 1..4) {
            let p = (p2 + (i + j + k)) % 10;
            let (x, y) = dfs(p1, p1_score, p, p2_score + p + 1, !p1_turn);
            p1_win += x;
            p2_win += y;
        }
    }
    (p1_win, p2_win)
}

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).unwrap();
    let mut p1: u64 = line.trim().split(": ").last().unwrap().parse().unwrap();
    p1 -= 1;
    line.clear();

    io::stdin().read_line(&mut line).unwrap();
    let mut p2: u64 = line.trim().split(": ").last().unwrap().parse().unwrap();
    p2 -= 1;

    let (p1_win, p2_win) = dfs(p1, 0, p2, 0, true);
    println!("{}", max(p1_win, p2_win));
}
