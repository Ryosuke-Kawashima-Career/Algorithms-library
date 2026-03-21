use proconio::input;
const INF: usize = 1 << 60;
// abc168d
// 幅優先探索
fn main() {
    input!{n: usize, m: usize}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..m {
        input!{a: usize, b: usize}
        let (a, b) = (a-1, b-1);
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut que = std::collections::VecDeque::new();
    que.push_back(0);
    let mut distance: Vec<usize> = vec![INF; n];
    distance[0] = 0;
    // Dpの復元にも応用できる。
    let mut pre_vertex: Vec<Vec<usize>> = vec![vec![]; n];

    while let Some(v) = que.pop_front() {
        for &next_v in graph[v].iter() {
            if distance[next_v] > distance[v] + 1 {
                distance[next_v] = distance[v] + 1;
                que.push_back(next_v);
                // 更新元の頂点を追加する。
                pre_vertex[next_v].push(v);
            }
        }
    }

    if distance.iter().any(|&x| x == INF) {
        println!("No");
    } else {
        println!("Yes");
        // 原点から各頂点に向かう探索をした。
        // 今度は各頂点から原点に向かうので、逆流する。つまりpre_vertexを使う。
        for v in 1..n {
            let pre_v: usize = pre_vertex[v][0] + 1;
            println!("{}", pre_v);
        }
    }
}