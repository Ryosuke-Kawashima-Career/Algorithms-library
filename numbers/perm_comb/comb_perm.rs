use proconio::{input, marker::Usize1};
use std::collections::HashSet;
use itertools::Itertools;
// abc054C
// Q. 始点から同じノードを複数回通らずにすべてのノードを回れるか?
// A. comb[perm[index]] = node
fn main() {
    input!{n: usize, m: usize, edges: [(Usize1, Usize1); m]}
    let mut graph: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    // 無向グラフ
    for &(a, b) in edges.iter() {
        graph[a].insert(b);
        graph[b].insert(a);
    }

    let mut ans: usize = 0;
    for length in n-1..=n-1 {
        // comb: 実際に選ばれる点
        for comb in (1..n).combinations(length) {
            // perm: 点の順序
            for perm in (0..length).permutations(length) {
                let mut all_ok = true;
                let mut seen = vec![false; n];
                seen[0] = true;
                // cur_posを定義することで始点が0からでもif文の例外を作る必要がなくなる
                let mut cur_pos: usize = 0;
                for i in 0..length {
                    if !graph[cur_pos].contains(&comb[perm[i]]) 
                    || seen[comb[perm[i]]] {
                        all_ok = false;
                    }
                    cur_pos = comb[perm[i]];
                    seen[comb[perm[i]]] = true;
                }
                // 同じ頂点を二度通らず，すべての頂点を訪れたか?
                if all_ok && seen == vec![true; n] {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}