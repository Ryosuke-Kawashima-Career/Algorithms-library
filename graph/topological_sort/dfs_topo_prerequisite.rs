use proconio::{input, marker::Usize1};
// abc315E
// トポロジカルソート(エッジの向きがいつもの逆)
// 依存関係を整理できる
fn main() {
    input!{n: usize}
    // 終点0から,さかのぼるのでエッジの向きは逆
    let graph: Vec<Vec<usize>> = input_data(n);

    let mut order: Vec<usize> = Vec::new();
    let mut seen: Vec<bool> = vec![false; n];
    // dfsで実装するとorderが逆になるので，逆の逆は正順
    topological_sort(0, &graph, &mut order, &mut seen);
    
    print_order(&order);
}

fn topological_sort(v: usize, graph: &Vec<Vec<usize>>, order: &mut Vec<usize>, seen: &mut Vec<bool>) {
    seen[v] = true;
    for &next in graph[v].iter() {
        if !seen[next] {
            topological_sort(next, graph, order, seen);
        }
    }
    // 帰りがけに記録する
    order.push(v);
}

fn input_data(n: usize) -> Vec<Vec<usize>> {
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for v in 0..n {
        input!{c: usize}
        for _ in 0..c {
            // p1,.., pc -> v
            input!{p: Usize1}
            // グラフのエッジの向きをいつもの逆にする
            graph[v].push(p);
        }
    }
    return graph;
}

fn print_order(order: &Vec<usize>) {
    for &ans in order.iter() {
        if ans == 0 {
            break;
        }
        print!("{} ", ans+1);
    }
    println!("");
}