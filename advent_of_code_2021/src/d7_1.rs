use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut nums = line
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    nums.sort();

    let med = nums[nums.len() / 2];
    let ans: i64 = nums.into_iter().map(|x| (med - x).abs()).sum();
    println!("{}", ans);
}
