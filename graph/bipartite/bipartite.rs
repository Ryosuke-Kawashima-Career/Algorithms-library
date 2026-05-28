#![allow(non_snake_case)]
use proconio::input;
// abc327d
// 二部グラフ判定
fn main() {
    input!{n: usize, m: usize, A: [usize; m], B: [usize; m]}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];

    for i in 0..m {
        graph[A[i]-1].push(B[i]-1);
        graph[B[i]-1].push(A[i]-1);
    }
    // 0, 1: color, 2: not visited
    let mut colors: Vec<usize> = vec![2; n];
    let mut is_bipartite: bool = true;
    // 各頂点を探索する。
    for v in 0..n {
        // if visited
        if colors[v] != 2 {
            continue;
        }
        if !judge_bipartite(v, 0, &graph, &mut colors) {
            is_bipartite = false;
        }
    }

    if is_bipartite {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn judge_bipartite(v: usize, color: usize, graph: &Vec<Vec<usize>>, colors: &mut Vec<usize>) -> bool {
    // 行きがけに色付けする
    colors[v] = color;

    for &next in graph[v].iter() {
        // if visited
        if colors[next] != 2 {
            if colors[next] == color {
                return false;
            }
            // 色が確定しているときは探索しない。
            // do not forget!!!
            continue;
        }
        // do not forget!!!
        if !judge_bipartite(next, (color + 1) % 2, graph, colors) {
            return false;
        }
    }

    return true;
}