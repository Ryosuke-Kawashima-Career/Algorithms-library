use proconio::{input, marker::Usize1};
const INF: i64 = 1 << 60;
// abc340D: dijkstra by Heap!!!
// VecDeque is not fast enough!!!
fn main() {
    input!{n: usize, abx: [(i64, i64, Usize1); n-1]}
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for i in 0..n-1 {
        graph[i].push((i+1, abx[i].0));
        graph[i].push((abx[i].2, abx[i].1));
    }
    let dist: Vec<i64> = dijkstra(0, &graph);
    println!("{}", dist[n-1]);
}

// priority que: (min_distance, vertex)
fn dijkstra(start: usize, graph: &Vec<Vec<(usize, i64)>>) -> Vec<i64> {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;

    let n: usize = graph.len();
    let mut dist: Vec<i64> = vec![INF; n];
    dist[start] = 0;
    let mut que = BinaryHeap::new();
    // (cur dist, cur vertex)
    que.push((Reverse(0), start));
    
    while let Some((Reverse(cur_cost), v)) = que.pop() {
        for &(next, cost) in graph[v].iter() {
            let next_cost = cur_cost + cost;
            if dist[next] > next_cost {
                dist[next] = next_cost;
                que.push((Reverse(next_cost), next));
            }
        }
    }

    return dist;
}

fn dijkstra(start: usize, graph: &Vec<Vec<(usize, i64)>>) -> Vec<i64> {
    let n: usize = graph.len();
    let mut dist: Vec<i64> = vec![INF; n];
    dist[start] = 0;
    let mut que = std::collections::BinaryHeap::new();
    que.push((std::cmp::Reverse(0), start));

    // pop vertex of min distance from the start
    while let Some((std::cmp::Reverse(cur_dist), vertex)) = que.pop() {
        for &(next, cost) in graph[vertex].iter() {
            if dist[next] > cur_dist + cost {
                let next_dist: i64 = cur_dist + cost;
                dist[next] = next_dist;
                que.push((std::cmp::Reverse(next_dist), next));
            }
        }
    }

    return dist;
}