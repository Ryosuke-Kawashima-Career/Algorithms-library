use proconio::{input, marker::Usize1};
const INF: usize = 1 << 60;
// ABC428E
// Q. the number of the vertex that has the maximum distance from vertex v ?
// A. Diameter of the tree: 木の直径を利用した解法
// Good Quality of diameters: 2 点間の距離が直径と一致するような頂点の組を選び (s,t) とする。(候補は複数ある場合は自由に 1 組選んでよい) この時、木に含まれる任意の頂点uについて、sとtの少なくとも一方がuから最も遠い頂点
// A. Expand the graph: グラフのノード数を倍にする = 頂点のコピーを作り、ノードの番号に合わせて辺の長さを決定する。
fn main() {
    input!{n: usize, edges: [(Usize1, Usize1); n-1]}
    let mut tree: Vec<Vec<usize>> = vec![vec![]; n];
    for &(u, v) in edges.iter() {
        tree[u].push(v);
        tree[v].push(u);
    }
    // get the diameter and its two ends.
    let dist_from_0: Vec<usize> = bfs(0, &tree);
    let end1: usize = arg_index_max(&dist_from_0);
    let dist_from_end1: Vec<usize> = bfs(end1, &tree);
    let end2: usize = arg_index_max(&dist_from_end1);
    let dist_from_end2: Vec<usize> = bfs(end2, &tree);

    for v in 0..n {
        if dist_from_end1[v] > dist_from_end2[v] {
            println!("{}", end1+1);
        } else if dist_from_end1[v] == dist_from_end2[v] {
            println!("{}", end1.max(end2) + 1);
        } else {
            println!("{}", end2+1);
        }
    }
}

fn arg_index_max(vector: &Vec<usize>) -> usize {
    let n: usize = vector.len();
    let mut max_index: usize = n - 1;
    for index in (0..n).rev() {
        if vector[index] > vector[max_index] {
            max_index = index;
        }
    }
    return max_index;
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