use proconio::{input, marker::Usize1};
use std::collections::VecDeque;
// ABC445C
// Q. 10^100 transfer operations and then print the final destinations
// Key Point: i <= A[i] <= n
// A. Reverse graph and BFS
/* Since the graph is a functional graph (out-degree 1) leading to fixed points, the components are trees rooted at fixed points (in reverse). */
fn main() {
    input! {n: usize, a: [Usize1; n]}
    // Reverse graph: reverse_graph[v] contains all u such that a[u] == v
    let mut reverse_graph: Vec<Vec<usize>> = vec![vec![]; n];
    for u in 0..n {
        let v = a[u];
        reverse_graph[v].push(u);
    }

    let destinations = bfs(n, &a, &reverse_graph);
    for v in 0..n {
        print!("{} ", destinations[v] + 1);
    }
    println!("");
}

fn bfs(n: usize, a: &[usize], reverse_graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut destinations: Vec<usize> = vec![n; n];
    let mut que = VecDeque::new();

    // Initialize with fixed points
    for v in 0..n {
        if a[v] == v {
            que.push_back(v);
            destinations[v] = v;
        }
    }

    while let Some(v) = que.pop_front() {
        // v reaches known fixed point destinations[v]
        // Neighbors next_v in reverse graph (where a[next_v] == v) also reach destinations[v]
        for &next_v in &reverse_graph[v] {
            if destinations[next_v] == n {
                // Not visited
                destinations[next_v] = destinations[v];
                que.push_back(next_v);
            }
        }
    }
    destinations
}
