use proconio::input;
const INF: usize = 1 << 60;

// 部分木のサイズを求める。鉄則B65
fn main() {
    input!{n: usize, t: usize}
    let t: usize = t-1;
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 1..n {
        input!{a: usize, b: usize}
        graph[a-1].push(b-1);
        graph[b-1].push(a-1);
    }
    let mut dp: Vec<usize> = vec![INF; n];
    subtree(t, &graph, &mut dp);

    for i in 0..n {
        print!("{} ", dp[i]);
    }
}

// distance == INF <=> not visited yet
fn subtree(parent: usize, graph: &Vec<Vec<usize>>, size: &mut Vec<usize>) {
    // visit parent
    size[parent] = 0;
    for &next in graph[parent].iter() {
        let mut size_parent: usize = size[parent];
        // if not visited
        if size[next] == INF {
            subtree(next, graph, size);
            let size_next: usize = size[next];
            // 初期値の0から大きくする。
            size_parent = size_parent.max(size_next+1); 
        }
        size[parent] = size_parent;
    }
}