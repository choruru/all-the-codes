use std::{cmp, io};

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let nums = line
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();

    let mut ans: i64 = i64::MAX;
    for n in min..=max {
        let sum = nums
            .iter()
            .map(|x| -> i64 {
                let diff = (n - x).abs();
                (diff * (diff + 1)) / 2
            })
            .sum();
        ans = cmp::min(ans, sum);
    }
    println!("{}", ans);
}
