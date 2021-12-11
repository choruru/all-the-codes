use std::collections::HashSet;
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
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>(),
        );
        line.clear();
    }

    let neighbors_yx = &vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut count = 0;
    let y_len = map.len();
    let x_len = map[0].len();
    for _n in 0..100 {
        // increment map
        for i in 0..y_len {
            for j in 0..x_len {
                map[i][j] += 1;
            }
        }

        let mut flashed = HashSet::new();
        // keep updating until no flash
        loop {
            let mut diff = vec![vec![0; map[0].len()]; map.len()];
            let mut no_update = true;
            // calc diff
            for i in 0..y_len {
                for j in 0..x_len {
                    let key = format!("{}:{}", i, j);
                    if map[i][j] > 9 && !flashed.contains(&key) {
                        no_update = false;
                        flashed.insert(key);
                        for (y, x) in neighbors_yx.into_iter() {
                            let ny = i as i32 + y;
                            let nx = j as i32 + x;
                            if 0 <= ny && ny < y_len as i32 && 0 <= nx && nx < x_len as i32 {
                                diff[ny as usize][nx as usize] += 1;
                            }
                        }
                    }
                }
            }

            if no_update {
                break;
            };

            // update map
            for i in 0..y_len {
                for j in 0..x_len {
                    map[i][j] += diff[i][j];
                }
            }
        }

        // set flashed positions to 0
        for i in 0..y_len {
            for j in 0..x_len {
                if map[i][j] > 9 {
                    map[i][j] = 0;
                };
            }
        }

        count += flashed.len();
    }

    println!("{:?}", count);
}
