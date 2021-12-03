use std::{io, io::prelude::*};

fn get_rating(lines: &Vec<String>, oxygen: bool) -> i64 {
    let mut lines: Vec<Vec<char>> = lines.iter().map(|x| x.trim().chars().collect()).collect();
    for i in 0..lines[0].len() {
        if lines.len() == 1 {
            break;
        }
        let ones_minus_zeros: i32 = lines
            .iter()
            .map(|line| if line[i] == '1' { 1 } else { -1 })
            .sum();
        let filter = if oxygen {
            ones_minus_zeros >= 0
        } else {
            ones_minus_zeros < 0
        };
        lines = lines
            .into_iter()
            .filter(|line| line[i] == if filter { '1' } else { '0' })
            .collect();
    }
    i64::from_str_radix(&lines[0].iter().collect::<String>(), 2).unwrap()
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|x| x.unwrap()).collect();
    println!("{}", get_rating(&lines, true) * get_rating(&lines, false));
}
