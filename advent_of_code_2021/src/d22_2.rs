use std::cmp::{max, min};
use std::collections::HashMap;
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
        ranges.push((on, xr[0], xr[1], yr[0], yr[1], zr[0], zr[1]));
        line.clear();
    }

    let mut cubes = HashMap::new();
    for r in ranges {
        let (on, xs1, xe1, ys1, ye1, zs1, ze1) = r;
        for ((xs2, xe2, ys2, ye2, zs2, ze2), sign) in cubes.clone() {
            let big_xs = max(xs1, xs2);
            let small_xe = min(xe1, xe2);
            let big_ys = max(ys1, ys2);
            let small_ye = min(ye1, ye2);
            let big_zs = max(zs1, zs2);
            let small_ze = min(ze1, ze2);
            if small_xe >= big_xs && small_ye >= big_ys && small_ze >= big_zs {
                // 0 clear
                *cubes
                    .entry((big_xs, small_xe, big_ys, small_ye, big_zs, small_ze))
                    .or_insert(0) -= sign;
            }
        }
        if on {
            *cubes.entry((xs1, xe1, ys1, ye1, zs1, ze1)).or_insert(0) += 1;
        }
    }

    let ans: i64 = cubes
        .into_iter()
        .map(|((xs, xe, ys, ye, zs, ze), sign)| {
            sign * (xe + 1 - xs) * (ye + 1 - ys) * (ze + 1 - zs)
        })
        .sum();

    println!("{}", ans);
}
