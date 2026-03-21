use proconio::{input, marker::Usize1};
// abc417E
// Q. find the lexicographically smallest among the integer sequences from Start to Goal
// A. DFS: Depth First Search
fn main() {
    input!{t: usize}
    let mut answers: Vec<Vec<usize>> = Vec::new();
    for _case in 0..t {
        input!{n: usize, m: usize, x_start: Usize1, y_end: Usize1, uv_edges: [(Usize1, Usize1); m]}
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        for &(u, v) in uv_edges.iter() {
            graph[u].push(v);
            graph[v].push(u);
        }
        for node in 0..n {
            graph[node].sort();
        }
        // path: records the path to the goal, answer: lexicographically smallest
        let mut seen: Vec<bool> = vec![false; n];
        let mut path: Vec<usize> = Vec::new();
        let mut ans: Vec<usize> = Vec::new();
        dfs(x_start, y_end, &graph, &mut seen, &mut path, &mut ans);
        answers.push(ans);
    }

    for path in answers {
        let n_path: usize = path.len();
        for i_node in 0..n_path {
            print!("{} ", path[i_node] + 1);
        }
        println!("");
    }
}

fn dfs(v: usize, end: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, path: &mut Vec<usize>, ans: &mut Vec<usize>) {
    let _n: usize = graph.len();
    path.push(v);
    seen[v] = true;
    if v == end {
        // Record the path
        *ans = path.clone();
        return;
    }

    for &next in graph[v].iter() {
        if seen[next] {
            continue;
        }
        dfs(next, end, graph, seen, path, ans);
    }
    
    // v is unnecessary!!!
    path.pop();
}