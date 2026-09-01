// TODO: Implement the required traits for `Item`.
// You can use the derive macro to automatically implement Eq and PartialEq.
// You will have to implement Ord and PartialOrd yourself,
// because the default implementation will rank `Item` based on `cost`
// (making a binary heap behave as a max heap), but we need a min heap instead.
//
// Hint: Given two Items `self` and `other`, you can implement the default Ord
// implementation by using `self.cost.cmp(&other.cost)` in your custom `cmp()` method.
// All you need to do is reverse this logic.
// Then, for PartialOrd, everything is a total order, so you can always
// return a `Some` enum variant containing your `cmp()` result from the Ord implementation.
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

    // Start with initialising a binary heap with a single element (`start`, as per above).
    // Loop while the binary heap is not empty:
    //   Loop through every element in `graph[u]`, destructing it into `(v, w)`:
    //     Calculate the new cost (`cost + w`).
    //     If this new cost is less (better) than the existing answer (`dist[v]`),
    //     then we want to explore this path further, such that:
    //       We update our answer (`dist[v]`) to now contain our new cost,
    //       and we add a new item with this cost and node to our traversal frontier (binary heap).

    // Your code will have the following structure (fill in the TODO):
    /*
    let mut open = TODO;
    while let TODO {
        for &(TODO) in &graph[u] {
            let new_cost = TODO;
            if TODO {
                TODO = new_cost;
                open.push(TODO);
            }
        }
    }
    */


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

    // TODO: To challenge yourself, change the for loop to an implementation just using iterator methods.
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let line = get_integer_line();
        let u = line[0] as usize;
        let v = line[1] as usize;
        let cost = line[2];

        graph[u].push((v, cost));
        graph[v].push((u, cost));
    }

    let answer = sssp(&graph, 0);
    for (i, distance) in answer.into_iter().enumerate() {
        println!("Node {i}, Shortest Distance: {distance}.");
    }
}
