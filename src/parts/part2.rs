// TODO: Implement the required traits for `Item`.
struct Item {
    cost: u64,
    node: usize,
}

fn sssp(graph: &Vec<Vec<(usize, u64)>>, source: usize) -> Vec<u64> {
    let mut dist = vec![u64::MAX; graph.len()];

    let start = Item {
        cost: 0,
        node: source,
    };
    dist[start.node] = start.cost;

    // TODO: Implement Dijkstra's.

    dist
}

// This function is already implemented to grab a list of space-separated integers from user input.
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

    // TODO: Take in the graph from standard input using `get_integer_line`.
    let graph = vec![vec![]; n];
    (0..m).for_each(|_| todo!());

    let answer = sssp(&graph, 0);
    for (i, distance) in answer.into_iter().enumerate() {
        println!("Node {i}, Shortest Distance: {distance}.");
    }
}
