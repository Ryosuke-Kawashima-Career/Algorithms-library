use proconio::input;
// unionfind is ok, too
// 鉄則A62
fn main() {
    input!{n: usize, m: usize}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..m {
        input!{a: usize, b: usize}
        graph[a-1].push(b-1);
        graph[b-1].push(a-1);
    }
    let mut seen: Vec<bool> = vec![false; n];
    dfs(0, &graph, &mut seen);

    if seen.iter().all(|&x| x) {
        println!("The graph is connected.");
    } else {
        println!("The graph is not connected.");
    }
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>) {
    seen[v] = true;
    for &next in graph[v].iter() {
        if seen[next] {
            continue;
        }
        dfs(next, graph, seen);
    }
}