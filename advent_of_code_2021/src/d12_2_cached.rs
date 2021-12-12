#[macro_use]
extern crate cached;
use cached::UnboundCache;
use std::collections::{BTreeSet, HashMap};
use std::io;

cached_key! {
    DFS: UnboundCache<String, i32> = UnboundCache::new();
    Key = { format!("{}:{:?}:{}", from, visited, single_small_twice_visited) };
    // returns number of paths starting from `from` with visited state.
    fn dfs(
        from: String,
        edges: &HashMap<String, Vec<String>>,
        visited: &mut BTreeSet<String>,
        single_small_twice_visited: bool
    ) -> i32 = {
        if &from == "end" {
            return 1;
        }

        let mut count = 0;
        for to in edges[&from].iter() {
            if to == "start" {
                continue;
            }
            let small_cave = to.clone().chars().next().unwrap().is_ascii_lowercase();
            if small_cave {
                if visited.contains(to) {
                    if single_small_twice_visited {
                        continue;
                    } else {
                        count += dfs(to.to_string(), edges, visited, true);
                    }
                } else {
                    visited.insert(to.clone());
                    count += dfs(to.to_string(), edges, visited, single_small_twice_visited);
                    visited.remove(&to.clone());
                }
            } else {
                count += dfs(to.to_string(), edges, visited, single_small_twice_visited);
            }
        }
        count
    }
}

fn main() {
    let mut line = String::new();
    let mut edges = HashMap::new();
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }
        let vs = line.trim().split("-").collect::<Vec<&str>>();
        let to = edges.entry(vs[0].to_string()).or_insert(vec![]);
        to.push(vs[1].to_string());
        let to = edges.entry(vs[1].to_string()).or_insert(vec![]);
        to.push(vs[0].to_string());
        line.clear();
    }

    let ans = dfs("start".to_string(), &edges, &mut BTreeSet::new(), false);
    println!("{}", ans);
}
