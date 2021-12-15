use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::io;

fn main() {
    let mut line = String::new();
    let mut map = vec![];
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }
        let l = line
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>();
        map.push(l);
        line.clear();
    }

    let ylen = map.len();
    let xlen = map[0].len();

    // edge construction
    let mut node_to_edges = HashMap::new();
    for y in 0..ylen {
        for x in 0..xlen {
            let node = format!("{}:{}", y, x);
            let edges = node_to_edges.entry(node.to_string()).or_insert(vec![]);
            if y > 0 {
                let up = format!("{}:{}", y - 1, x);
                edges.push((up, map[y - 1][x]));
            }
            if y + 1 < ylen {
                let down = format!("{}:{}", y + 1, x);
                edges.push((down, map[y + 1][x]));
            }
            if x > 0 {
                let left = format!("{}:{}", y, x - 1);
                edges.push((left, map[y][x - 1]));
            }
            if x + 1 < xlen {
                let right = format!("{}:{}", y, x + 1);
                edges.push((right, map[y][x + 1]));
            }
        }
    }

    // Dijkstra, O(V logE)

    // stores best cost of each node
    let mut dist = HashMap::new();
    // best cost of the start pos is 0
    let _ = dist.entry("0:0".to_string()).or_insert(0);

    let mut que = BinaryHeap::new();
    // (cost, node) so that cost will be the first key of comparison.
    que.push(Reverse((0, "0:0".to_string())));
    const INF: i32 = 100_000_000;
    while let Some(entry) = que.pop() {
        // unwrap Reverse
        let (best_cost_candidate, node) = entry.0;

        // filter out candidates
        // don't wanna borrow mut ref of dict.
        let _ = dist.entry(node.to_string()).or_insert(INF);
        let best_cost = dist[&node];
        if best_cost < best_cost_candidate {
            continue;
        }

        // create next candidates
        for (next_node, edge_cost) in node_to_edges[&node.to_string()].iter() {
            let best_cost_of_next_node = dist.entry((&next_node).to_string()).or_insert(INF);
            if *best_cost_of_next_node > best_cost + edge_cost {
                *best_cost_of_next_node = best_cost + edge_cost;
                que.push(Reverse((*best_cost_of_next_node, (&next_node).to_string())));
            }
        }
    }

    let end = format!("{}:{}", ylen - 1, xlen - 1);
    println!("{:#?}", dist[&end]);
}
