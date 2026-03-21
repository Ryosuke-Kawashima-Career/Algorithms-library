use proconio::input;
// 鉄則B62
fn main() {
    input!{n: usize, m: usize}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..m {
        input!{a: usize, b: usize}
        graph[a-1].push(b-1);
        graph[b-1].push(a-1);
    }

    let mut seen: Vec<bool> = vec![false; n];
    let mut order: Vec<usize> = Vec::new();
    dfs_record(0, &graph, &mut seen, &mut order);

    let length: usize = order.len();
    for i in 1..=length {
        print!("{} ", order[length-i]+1);
    }
}

// 逆順に記録する．
fn dfs_record(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, order: &mut Vec<usize>) -> bool {
    seen[v] = true;
    if v == graph.len() - 1 {
        order.push(graph.len() - 1);
        return true;
    }
    let mut is_connected: bool = false;
    for &next in graph[v].iter() {
        if seen[next] {
            continue;
        }
        if dfs_record(next, graph, seen, order) {
            is_connected = true;
            // 帰りがけに頂点を記録する
            order.push(v);
        }
    }

    return is_connected;
}