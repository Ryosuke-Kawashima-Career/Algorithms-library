use proconio::{input, marker::{Chars, Usize1}};
// abc427D
// Q. Generalized Min-Max if the node is A, then the fist player: Alice wins. Each performs K-turns => 2K turns in total.
// A. Think from the end!
// このようなゲームの問題では、後ろから勝敗の状態を解析するのが有効
// 各状態についてその状態が Alice が必勝の状態であるのか Bob が必勝の状態であるのか解析する
// dpi,jを Alice と Bob が合わせてi 回の操作を行い、駒が頂点j にあるときに Alice が必勝の状態であるかの真偽値を表す変数とする。
fn main() {
    input!{t: usize}
    for _case in 0..t {
        input!{n: usize, m: usize, k: usize, s: Chars, edges: [(Usize1, Usize1); m]}
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        for &(u, v) in edges.iter() {
            graph[u].push(v);
        }
        if judge_alice_victory(k, &s, &graph) {
            println!("Alice");
        } else {
            println!("Bob");
        }
    }
}

fn judge_alice_victory(turns: usize, nodes: &Vec<char>, graph: &Vec<Vec<usize>>) -> bool {
    let n: usize = nodes.len();
    // dp[remaining turns][nodes]
    let mut dp_can_alice_win: Vec<Vec<bool>> = vec![vec![false; n]; 2*turns+1];
    for v in 0..n {
        if nodes[v] == 'A' {
            dp_can_alice_win[0][v] = true;
        } else {
            dp_can_alice_win[0][v] = false;
        }
    }

    for remaining_turns in 1..=turns {
        // Bob's Turn
        for v in 0..n {
            dp_can_alice_win[2*remaining_turns-1][v] = true;
        }
        for v in 0..n {
            for &next in graph[v].iter() {
                if !dp_can_alice_win[2*(remaining_turns-1)][next] {
                    dp_can_alice_win[2*remaining_turns-1][v] = false;
                }
            }
        }
        // Alice's Turn
        for v in 0..n {
            dp_can_alice_win[2*remaining_turns][v] = false;
        }
        for v in 0..n {
            for &next in graph[v].iter() {
                // At least, Alice has one chance to win.
                if dp_can_alice_win[2*remaining_turns-1][next] {
                    dp_can_alice_win[2*remaining_turns][v] = true;
                }
            }
        }
    }

    // "v=0" = "Being on the initial node"
    return dp_can_alice_win[2*turns][0];
}