use std::io;

fn main() {
    let mut line = String::new();
    let mut prev = -1;
    let mut count = 0;
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }
        let curr = line.trim().parse().unwrap();
        if prev != -1 && prev < curr {
            count += 1;
        }
        prev = curr;
        line.clear();
    }
    println!("{}", count);
}
