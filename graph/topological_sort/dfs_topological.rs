use proconio::input;
// calc path-length EducationalDP-G
// only used for DAG(Directed Acyclic Graph)
fn main() {
    input!{n: usize, m: usize}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..m {
        input!{x: usize, y: usize}
        graph[x-1].push(y-1);
    }
    let mut dp: Vec<usize> = vec![0; n];
    // トポロジカルソート
    let mut order: Vec<usize> = Vec::new();
    let mut is_visited: Vec<bool> = vec![false; n];
    for i in 0..n {
        if !is_visited[i] {
            topologicalsort(i, &graph, &mut order, &mut is_visited);
        }
    }
    order.reverse();

    dp[order[0]] = 0;
    for i in 0..n {
        for &next in graph[order[i]].iter() {
            dp[next] = dp[next].max(dp[order[i]] + 1);
        }
    }
    let mut ans: usize = *dp.iter().max().unwrap();
    println!("{}", ans);
}

fn topologicalsort(v: usize, graph: &Vec<Vec<usize>>, order: &mut Vec<usize>, seen: &mut Vec<bool>) {
    // if visited
    seen[v] = true;

    for &next in graph[v].iter() {
        if seen[next] {
            continue;
        }
        // 帰りがけの再帰関数(子の情報を使う)
        topologicalsort(next, graph, order, seen);
    }

    // update!
    order.push(v);
}