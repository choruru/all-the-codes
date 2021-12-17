use std::cmp::max;
use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let mut iter = line
        .split("x=")
        .last()
        .unwrap()
        .split(",")
        .next()
        .unwrap()
        .split("..")
        .map(|x| x.parse().unwrap());
    let left: i32 = iter.next().unwrap();
    let right: i32 = iter.next().unwrap();

    let mut iter = line
        .split("y=")
        .last()
        .unwrap()
        .split("..")
        .map(|x| x.parse().unwrap());
    let low: i32 = iter.next().unwrap();
    let high: i32 = iter.next().unwrap();

    assert!(left > 0);
    let x_min = 1;
    let x_max = right;

    assert!(low < 0);
    let y_min = low;
    let y_max = -(low + 1);

    let mut ans = 0;
    for x in x_min..=x_max {
        for y in y_min..=y_max {
            let mut x_pos = 0;
            let mut y_pos = 0;
            let mut x_vel = x;
            let mut y_vel = y;

            loop {
                x_pos += x_vel;
                x_vel = max(x_vel - 1, 0);
                y_pos += y_vel;
                y_vel -= 1;

                if x_pos > right || y_pos < low {
                    break;
                }
                if left <= x_pos && x_pos <= right && low <= y_pos && y_pos <= high {
                    ans += 1;
                    break;
                }
            }
        }
    }

    println!("{}", ans);
}
