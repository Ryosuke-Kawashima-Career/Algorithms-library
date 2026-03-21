use proconio::input;
const INF: usize = 1 << 60;
// 鉄則A70
// 状態を2進法で表し、グラフの最短経路問題に帰着する。
fn main() {
    input!{n: usize, m: usize, a: [usize; n], xyz: [(usize, usize, usize); m]}
    let states: usize = 1 << n;
    let mut graph: Vec<Vec<usize>> = vec![vec![]; states];

    for state in 0..states {
        for switch in 0..m {
            let (x, y, z) = xyz[switch];
            let (x, y, z) = (x-1, y-1, z-1);
            // switchのon-offの反転をxorで表現する。
            let next_state: usize = state ^ (1 << x) ^ (1 << y) ^ (1 << z);
            graph[state].push(next_state);
        }
    }

    let mut start: usize = 0;
    for i in 0..n {
        if a[i] == 1 {
            start |= 1 << i;
        }
    }
    let goal: usize = states-1;

    let dist: Vec<usize> = bfs(start, &graph);
    let min_length: usize = dist[goal];
    if min_length == INF {
        println!("-1");
    } else {
        println!("{}", min_length);
    }
}

fn bfs(start: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let n: usize = graph.len();
    let mut dist: Vec<usize> = vec![INF; n];
    dist[start] = 0;
    let mut que = std::collections::VecDeque::new();
    que.push_back(start);

    while let Some(vertex) = que.pop_front() {
        for &next in graph[vertex].iter() {
            if dist[next] > dist[vertex] + 1 {
                dist[next] = dist[vertex] + 1;
                que.push_back(next);
            }
        }
    }

    return dist;
}