use std::{
    cmp::{Ord, Ordering, PartialOrd},
    collections::BinaryHeap,
};

#[derive(Eq, PartialEq)]
struct Item {
    cost: u64,
    node: usize,
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn sssp(graph: &Vec<Vec<(usize, u64)>>, source: usize) -> Vec<u64> {
    let mut dist = vec![u64::MAX; graph.len()];
    dist[source] = 0;

    let start = Item {
        cost: 0,
        node: source,
    };
    let mut open = BinaryHeap::from([start]);

    while let Some(Item { cost, node: u }) = open.pop() {
        for &(v, w) in &graph[u] {
            let cost = cost + w;
            if cost < dist[v] {
                dist[v] = cost;
                open.push(Item { cost, node: v });
            }
        }
    }

    dist
}

fn get_integer_line() -> Vec<u64> {
    let mut buffer: String = Default::default();
    let _ = std::io::stdin().read_line(&mut buffer);
    buffer
        .split_whitespace()
        .filter_map(|num| num.parse::<u64>().ok())
        .collect()
}

pub fn main() {
    let first_line = get_integer_line();
    let (n, m) = (first_line[0] as usize, first_line[1] as usize);

    let mut graph = vec![vec![]; n];
    (0..m)
        .flat_map(|_| {
            let line = get_integer_line();
            [(line[0], (line[1], line[2])), (line[1], (line[0], line[2]))]
        })
        .for_each(|(u, (v, cost))| {
            graph[u as usize].push((v as usize, cost));
        });

    let answer = sssp(&graph, 0);

    for (i, distance) in answer.into_iter().enumerate() {
        println!("Node {i}, Shortest Distance: {distance}.");
    }
}
