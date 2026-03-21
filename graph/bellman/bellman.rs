use proconio::input;

const INF: i64 = 1 << 60;

#[derive(Debug, Copy, Clone)]
struct Edge {
    to: usize, cost: i64
}

impl Edge {
    fn new(node: usize, length: i64) -> Self {
        Self{to: node, cost: length}
    }
}
// abc061D
// one way?
fn main() {
    input!{n: usize, m: usize}
    let mut graph: Vec<Vec<Edge>> = vec![vec![]; n];
    for _ in 0..m {
        input!{a: usize, b: usize, c: i64}
        graph[a-1].push(Edge::new(b-1, c));
    }
    let mut dist: Vec<i64> = vec![-INF; n];
    // initialization!!!
    dist[0] = 0;

    let mut is_updated: bool = false;
    for iter in 0..n {
        for i in 0..n {
            // not always needed
            if dist[i] == -INF {
                continue;
            }
            for &Edge{to, cost} in graph[i].iter() {
                if dist[to] < dist[i] + cost {
                    dist[to] = dist[i] + cost;
                    // check vertex:to has an infinite cycle!!!
                    if iter == n-1 && to == n-1 {
                        is_updated = true;
                    }
                }
            }
        }
    }

    if is_updated {
        println!("inf");
    } else {
        println!("{}", dist[n-1]);
    }
}