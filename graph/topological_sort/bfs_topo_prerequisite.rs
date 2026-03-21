use proconio::{input, marker::Usize1};
// abc315E
// トポロジカルソート(エッジの向きがいつもの逆)
// 依存関係を整理できる
fn main() {
    input!{n: usize}
    // 終点0から,さかのぼるのでエッジの向きは逆
    let graph: Vec<Vec<usize>> = input_data(n);

    let order: Vec<usize> = topological_sort(&graph);
    let mut order_from0 = reachable_from(0, &graph, &order);
    // bfsで実装するとエッジの向きは逆のまま
    order_from0.reverse();
    print_order(&order_from0);
}

fn topological_sort(graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let n: usize = graph.len();
    let mut order: Vec<usize> = Vec::new();
    let mut in_degrees: Vec<usize> = vec![0; n];
    for v in 0..n {
        for &next in graph[v].iter() {
            in_degrees[next] += 1;
        }
    }
    let mut zero_degrees = std::collections::VecDeque::new();
    for v in 0..n {
        if in_degrees[v] == 0 {
            zero_degrees.push_back(v);
        }
    }

    while let Some(v) = zero_degrees.pop_front() {
        order.push(v);
        for &next in graph[v].iter() {
            // erase edge
            in_degrees[next] -= 1;
            if in_degrees[next] == 0 {
                zero_degrees.push_back(next);
            }
        }
    }
    return order;
}

// 幅優先探索でv0と接続の頂点を求める
fn reachable_from(v0: usize, graph: &Vec<Vec<usize>>, order: &Vec<usize>) -> Vec<usize> {
    let n: usize = graph.len();
    // 頂点のトポロジカルソート後のindexを格納する
    let mut indexes: Vec<usize> = vec![n; n];
    for i in 0..n {
        indexes[order[i]] = i;
    }
    // v0に到達可能な頂点を格納する
    let mut reachables: Vec<usize> = Vec::new();
    let mut que = std::collections::VecDeque::new();
    let mut seen: Vec<bool> = vec![false; n];
    seen[v0] = true;
    que.push_back(v0);
    
    while let Some(v) = que.pop_front() {
        reachables.push(v);
        for &next in graph[v].iter() {
            // seenを変更してからqueに入れることでqueの頂点のダブりを防ぐ
            if !seen[next] {
                seen[next] = true;
                que.push_back(next);
            }
        }
    }
    // トポロジカルソートの順番に並べる
    reachables.sort_by_key(|&x| indexes[x]);
    return reachables;
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