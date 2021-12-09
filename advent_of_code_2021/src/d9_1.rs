use std::io;

fn main() {
    let mut line = String::new();
    let mut map = vec![];
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }
        map.push(
            line.trim()
                .chars()
                .map(|x| x.to_digit(10).unwrap() + 1)
                .collect::<Vec<u32>>(),
        );
        line.clear();
    }

    let mut count = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let x = map[i][j];
            let mut neighbors = vec![];
            if i > 0 {
                neighbors.push(map[i - 1][j]);
            }
            if i < map.len() - 1 {
                neighbors.push(map[i + 1][j]);
            }
            if j > 0 {
                neighbors.push(map[i][j - 1]);
            }
            if j < map[0].len() - 1 {
                neighbors.push(map[i][j + 1]);
            }
            if neighbors.iter().all(|nei| nei > &x) {
                count += x;
            }
        }
    }

    println!("{:?}", count);
}
