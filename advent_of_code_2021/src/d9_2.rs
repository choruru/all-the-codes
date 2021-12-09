use std::collections::VecDeque;
use std::io;

const VISITED: u32 = 100;

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
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>(),
        );
        line.clear();
    }

    let mut basins = vec![];
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let mut que = VecDeque::new();
            que.push_back(format!("{}:{}", i, j));
            let mut size = 0;
            while !que.is_empty() {
                let front = que.pop_front().unwrap();
                let mut yx = front.split(":");
                let y: usize = yx.next().unwrap().parse().unwrap();
                let x: usize = yx.next().unwrap().parse().unwrap();
                if let VISITED | 9 = map[y][x] {
                    continue;
                }
                map[y][x] = VISITED;
                size += 1;
                if y > 0 {
                    que.push_back(format!("{}:{}", y - 1, x));
                }
                if y < map.len() - 1 {
                    que.push_back(format!("{}:{}", y + 1, x));
                }
                if x > 0 {
                    que.push_back(format!("{}:{}", y, x - 1));
                }
                if x < map[0].len() - 1 {
                    que.push_back(format!("{}:{}", y, x + 1));
                }
            }
            if size > 0 {
                basins.push(size);
            }
        }
    }

    basins.sort_by(|a, b| b.cmp(a));
    println!("{:?}", basins[0] * basins[1] * basins[2]);
}
