use std::io;

fn main() {
    let mut line = String::new();
    let mut points = vec![];
    while let Ok(_) = io::stdin().read_line(&mut line) {
        if line == "\n" {
            line.clear();
            break;
        }
        let p = line
            .trim()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();
        points.push((p[0], p[1]));
        line.clear();
    }

    let mut commands: Vec<(char, usize)> = vec![];
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }
        let l = line.trim().split("=").collect::<Vec<&str>>();
        commands.push((l[0].chars().last().unwrap(), l[1].parse().unwrap()));
        line.clear();
    }

    let mut ylen= points.iter().map(|p| p.1).max().unwrap() + 1;
    let mut xlen= points.iter().map(|p| p.0).max().unwrap() + 1;
    let mut map = vec![vec![false; xlen]; ylen];
    for p in points.iter() {
        map[p.1][p.0] = true;
    }

    for (axis, val) in commands.into_iter() {
        let new_xlen = if axis == 'x' { val } else {xlen};
        let new_ylen = if axis == 'y' { val } else {ylen};
        let mut new_map = vec![vec![false; new_xlen]; new_ylen];
        for y in 0..new_ylen {
            for x in 0..new_xlen {
                new_map[y][x] = map[y][x];

                let y_mirror = 2 * new_ylen - y;
                if axis == 'y' && y_mirror < ylen {
                    new_map[y][x] |= map[y_mirror][x];
                }

                let x_mirror = 2 * new_xlen - x;
                if axis == 'x' && x_mirror < xlen {
                    new_map[y][x] |= map[y][x_mirror];
                } 
            }
        }
        map = new_map;
        ylen = new_ylen;
        xlen = new_xlen;
    }

    for line in map.iter() {
        println!("{:?}", line.iter().map(|x| if *x {"#"} else {"."}).collect::<String>());
    }
}
