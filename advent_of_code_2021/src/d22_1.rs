use itertools::iproduct;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::io;

fn main() {
    let mut line = String::new();
    // on x=10..12,y=10..12,z=10..12
    let mut ranges = vec![];
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }
        let l = line.trim();
        let on = l.starts_with("on");
        let pos = l.split(",").collect::<Vec<&str>>();
        let xr = pos[0]
            .split("=")
            .last()
            .unwrap()
            .split("..")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i64>>();
        let yr = pos[1]
            .split("=")
            .last()
            .unwrap()
            .split("..")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i64>>();
        let zr = pos[2]
            .split("=")
            .last()
            .unwrap()
            .split("..")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i64>>();
        let xmin = min(max(xr[0], -50), 50);
        let xmax = min(max(xr[1], -50), 50);
        let ymin = min(max(yr[0], -50), 50);
        let ymax = min(max(yr[1], -50), 50);
        let zmin = min(max(zr[0], -50), 50);
        let zmax = min(max(zr[1], -50), 50);
        if !(xmin == xmax || ymin == ymax || zmin == zmax) {
            ranges.push((on, xmin..=xmax, ymin..=ymax, zmin..=zmax));
        }
        line.clear();
    }

    // brute force
    let mut on = HashSet::new();
    for range in ranges {
        for (x, y, z) in iproduct!(range.1, range.2, range.3) {
            if range.0 {
                on.insert((x, y, z));
            } else {
                on.remove(&(x, y, z));
            }
        }
    }
    println!("{:#?}", on.len())
}
