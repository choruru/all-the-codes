use cached::proc_macro::cached;
use std::io;

#[cached]
fn solve(first_birth: i64) -> i64 {
    let mut children = 0;
    let mut day = first_birth;
    while day <= 256 {
        children += solve(day + 9);
        day += 7;
    }
    return 1 + children;
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    // The first birth after X days.
    let first_births = line
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap() + 1)
        .collect::<Vec<i64>>();

    let sum: i64 = first_births.iter().map(|x| solve(*x)).sum();
    println!("{}", sum);
}
