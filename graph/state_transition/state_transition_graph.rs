use proconio::{input, marker::Usize1};
const INF: usize = 1 << 60;
// 典型A70
// 状態の遷移をグラフで言い換える
// switchの操作xor
fn main() {
    // a[i: 0..n]: ランプiの初期値 a[i]= 0: off, 1: off
    // xyz[i: 0..m]: ボタンiを押すことでランプx,y,zのon-offを切り替える
    input!{n: usize, m: usize, a: [usize; n], xyz: [(Usize1, Usize1, Usize1); m]}
    // ランプのon-offのすべての状態
    let bits: usize = 1 << n;
    let mut graph: Vec<Vec<usize>> = vec![vec![]; bits];
    let mut start_state: usize = 0;
    // 初期状態をorで構築する
    for i in 0..n {
        if a[i] == 1 {
            start_state |= 1 << i;
        }
    }

    // bitはランプの状態
    for bit in 0..bits {
        // ボタンを押してランプを切り替える
        for &(x, y, z) in xyz.iter() {
            let next_state: usize = bit ^ (1 << x) ^ (1 << y) ^ (1 << z);
            graph[bit].push(next_state);
        }
    }
    let dist = bfs_state(start_state, &graph);
    if dist[bits - 1] == INF {
        println!("-1");
    } else {
        println!("{}", dist[bits - 1]);
    }
}

fn bfs_state(start: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
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