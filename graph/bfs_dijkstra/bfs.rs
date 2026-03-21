use proconio::input;
const INF: usize = 1 << 60;

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
// abc132e
fn main() {
    input!{n: usize, m: usize}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..m {
        input!{u: usize, v: usize}
        graph[u-1].push(v-1);
    }
    input!{s1: usize, t1: usize}
    let (s, t) = (s1 - 1, t1 - 1);
    // 三つの辺を同時に移動するので3
    let mut dist: Vec<Vec<usize>> = vec![vec![INF; 3]; n];
    let mut que = std::collections::VecDeque::new();
    // init
    dist[s][0] = 0;
    que.push_back((s, 0));

    while let Some((node, phase)) = que.pop_front() {
        // 隣接点を調べる。
        for &v in graph[node].iter() {
            if dist[v][(phase+1)%3] > dist[node][phase] + 1 {
                dist[v][(phase+1)%3] = dist[node][phase] + 1;
                que.push_back((v, (phase+1)%3));
            }
        }
    }

    if dist[t][0] == INF {
        println!("-1");
    } else {
        //　けんけんぱ
        println!("{}", dist[t][0] / 3);
    }
}