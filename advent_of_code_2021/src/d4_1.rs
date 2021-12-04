use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Board {
    num_to_pos: HashMap<usize, usize>,
    unopened_sum: i64,
    rows: Vec<i32>,
    cols: Vec<i32>,
}

impl Board {
    fn new() -> Self {
        Board {
            num_to_pos: HashMap::new(),
            unopened_sum: 0,
            rows: vec![5; 5],
            cols: vec![5; 5],
        }
    }

    fn is_full(&self) -> bool {
        self.num_to_pos.len() == 25
    }

    fn add_number(&mut self, n: usize) {
        self.num_to_pos.insert(n, self.num_to_pos.len());
        self.unopened_sum += n as i64;
    }

    fn open(&mut self, n: usize) -> Option<i64> {
        // return score
        let mut win = false;
        if let Some(pos) = self.num_to_pos.get(&n) {
            self.unopened_sum -= n as i64;
            // rows
            self.rows[pos / 5] -= 1;
            if self.rows[pos / 5] == 0 {
                win = true;
            }
            // cols
            self.cols[pos % 5] -= 1;
            if self.cols[pos % 5] == 0 {
                win = true;
            }
        }
        if win {
            Some(n as i64 * self.unopened_sum)
        } else {
            None
        }
    }
}

fn main() {
    let mut line = String::new();
    let mut nums = vec![];
    let mut boards = vec![];
    let mut board = Board::new();
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            boards.push(board);
            break;
        }
        if line == "\n" {
            ()
        } else if line.contains(",") {
            nums = line
                .trim()
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<usize>>();
        } else {
            if board.is_full() {
                boards.push(board);
                board = Board::new();
            }
            for n in line
                .trim()
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
            {
                board.add_number(n);
            }
        }
        line.clear();
    }

    for n in nums.iter() {
        for b in boards.iter_mut() {
            if let Some(x) = b.open(*n) {
                println!("{:?}", x);
                return;
            }
        }
    }
}
