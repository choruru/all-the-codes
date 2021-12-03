use std::io;

fn main() {
    let mut line = String::new();
    let mut nums = vec![];
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }
        let chars: Vec<char> = line.trim().chars().collect();
        line.clear();
        if nums.len() == 0 {
            nums = vec![0; chars.len()];
        }
        for i in 0..chars.len() {
            if chars[i] == '1' {
                nums[i] += 1;
            } else {
                nums[i] -= 1;
            }
        }
    }

    let mut gamma = 0;
    let mut eps = 0;
    for i in 0..nums.len() {
        if nums[i] > 0 {
            gamma += 1 << (nums.len() - 1 - i);
        } else {
            eps += 1 << (nums.len() - 1 - i);
        }
    }
    println!("{}", gamma * eps);
}
