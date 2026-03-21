use proconio::{input, marker::Usize1};
// abc311C
// Functional Graph: N頂点N辺の有向グラフであって、各頂点から丁度一つの辺が出ているグラフ
// 有向閉路: 始点と終点が同じ循環するパス
// 頂点を一つ任意に取り、そこから移動し続けることでサイクルに到達する
fn main() {
    input!{n: usize, a: [Usize1; n]}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for v in 0..n {
        graph[v].push(a[v]);
    }

    let order = detect_cycle(0, &graph);
    print_cycle(&order);
}

fn detect_cycle(start: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let n: usize = graph.len();
    let mut seen: Vec<bool> = vec![false; n];
    let mut visited_nodes: Vec<usize> = Vec::new();
    // 好きな頂点から探索を始める
    let mut cur_v: usize = start;
    while !seen[cur_v] {
        seen[cur_v] = true;
        visited_nodes.push(cur_v);
        // functional graph!!!
        cur_v = graph[cur_v][0];
    }

    // cur_v: cycleの出発点になる(なぜならseen[cur_v] = true)
    let start_of_cycle = cur_v;
    let mut cycle: Vec<usize> = Vec::new();
    let mut is_start_detected = false;
    for &node in visited_nodes.iter() {
        if node == start_of_cycle {
            is_start_detected = true;
        }
        if is_start_detected {
            cycle.push(node);
        }
    }
    return cycle;
}

fn print_cycle(cycle: &Vec<usize>) {
    let k: usize = cycle.len();
    println!("{}", k);
    for i in 0..k {
        print!("{} ", cycle[i]+1);
    }
    println!("");
}