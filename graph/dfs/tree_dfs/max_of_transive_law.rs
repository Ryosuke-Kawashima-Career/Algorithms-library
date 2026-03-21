use proconio::{input, marker::Usize1};
// abc313B
// 推移律: x > y && y > z <=> x > z
fn main() {
    input!{n: usize, m: usize, edges: [(Usize1, Usize1); m]}
    // 最強が木構造の根に位置するときに最強になる
    let mut tree: Vec<Vec<usize>> = vec![vec![]; n];
    for &(x, y) in edges.iter() {
        tree[x].push(y);
    }
    
    for v in 0..n {
        let mut seen: Vec<bool> = vec![false; n];
        // if subtree size is n, v is the champion
        if dfs(v, &tree, &mut seen) == n {
            println!("{}", v+1);
            return;
        }
    }
    println!("-1");
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>) -> usize {
    seen[v] = true;
    let mut size = 1;
    for &next in graph[v].iter() {
        if !seen[next] {
            let sub_size = dfs(next, graph, seen);
            size += sub_size;
        }
    }
    return size;
}