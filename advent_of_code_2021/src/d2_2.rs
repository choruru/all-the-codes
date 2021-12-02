use std::io;

fn main() {
    let mut line = String::new();
    let mut forward = 0;
    let mut depth = 0;
    let mut aim = 0;
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }
        let op = line.trim().split(' ').collect::<Vec<&str>>();
        let command = op[0];
        let d: i64 = op[1].parse().unwrap();
        match command {
            "forward" => {
                forward += d;
                depth += aim * d
            }
            "down" => aim += d,
            "up" => aim -= d,
            _ => panic!("Unexpected command"),
        }
        line.clear();
    }
    println!("{}", forward * depth);
}
