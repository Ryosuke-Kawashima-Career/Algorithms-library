use proconio::input;
// 典型39
// 木DP
// 主客転倒：答えへの貢献度を元に問題を言い換える。
fn main() {
    input!{n: usize}
    let mut tree: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 1..n {
        input!{a: usize, b: usize}
        tree[a-1].push(b-1);
        tree[b-1].push(a-1);
    }

    // 部分木の大きさを求める。(DP)
    let mut subtree_sizes: Vec<usize> = vec![0; n];
    subtree(0, &tree, &mut subtree_sizes);
    // 頂点間の距離の和
    let mut ans: usize = 0;

    for v in 0..n {
        for &next in tree[v].iter() {
            if v < next {
                let min_size: usize = subtree_sizes[v].min(subtree_sizes[next]);
                let edge_contribution: usize = min_size * (n - min_size);
                ans += edge_contribution;
            }
        }
    }

    println!("{}", ans);
}

fn subtree(v: usize, graph: &Vec<Vec<usize>>, subtree_sizes: &mut Vec<usize>) {
    subtree_sizes[v] += 1;
    for &next in graph[v].iter() {
        if subtree_sizes[next] != 0 {
            continue;
        }
        subtree(next, graph, subtree_sizes);
        let child_size: usize = subtree_sizes[next];
        subtree_sizes[v] += child_size;
    }
}