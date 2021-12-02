use std::io;

fn main() {
    let mut line = String::new();
    let mut nums = vec![0, 0, 0];
    let mut count = 0;
    let mut index = 0;
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }
        let curr = line.trim().parse().unwrap();
        let prev_sum: i32 = nums.iter().sum();
        nums[index % 3] = curr;
        let curr_sum = nums.iter().sum();
        if index > 2 && prev_sum < curr_sum {
            count += 1;
        }
        line.clear();
        index += 1;
    }
    println!("{}", count);
}
