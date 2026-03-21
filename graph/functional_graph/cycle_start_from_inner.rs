use proconio::{input, marker::Usize1};
// abc311C
// Functional Graph: N頂点N辺の有向グラフであって、各頂点から丁度一つの辺が出ているグラフ
// 有向閉路: 始点と終点が同じ循環するパス
// 頂点を一つ任意に取り、そこから移動し続けることでサイクルに到達する
// 予めサイクル内から始めることができれば、実装はより簡単
// 最初にN回移動しておくことで必ず達成(Aの先頭にダミーを増やして1-indexedにしておく)
fn main() {
    input!{n: usize, a: [Usize1; n]}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for v in 0..n {
        graph[v].push(a[v]);
    }

    let order = move_cycle(0, &graph);
    print_cycle(&order);
}

// 好きな頂点から始める
fn move_cycle(start: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let n: usize = graph.len();
    // 頂点の数だけ動くと，必ずループに囚われる!
    let mut cur_v = start;
    for _ in 0..n {
        cur_v = graph[cur_v][0];
    }

    let start_of_cycle = cur_v;
    let mut cycle: Vec<usize> = Vec::new();
    cycle.push(start_of_cycle);
    cur_v = graph[start_of_cycle][0];
    while cur_v != start_of_cycle {
        cycle.push(cur_v);
        cur_v = graph[cur_v][0];
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