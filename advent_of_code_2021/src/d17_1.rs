use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let low: i32 = line
        .split("y=")
        .last()
        .unwrap()
        .split("..")
        .next()
        .unwrap()
        .parse()
        .unwrap();

    assert!(low < 0);
    let n = -(low + 1);
    println!("{}", n * (n + 1) / 2);
}
