use std::io;

fn main() {
    let mut line = String::new();
    let mut ans = 0;
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }
        let parts = line.trim().split(" | ").collect::<Vec<&str>>();
        let outputs = parts[1]
            .split(" ")
            .map(|s| -> Vec<char> {
                let mut chars = s.chars().collect::<Vec<char>>();
                chars.sort();
                chars
            })
            .collect::<Vec<Vec<char>>>();

        for c in outputs.into_iter() {
            if let 2 | 3 | 4 | 7 = c.len() {
                ans += 1;
            }
        }
        line.clear();
    }
    println!("{:?}", ans);
}
