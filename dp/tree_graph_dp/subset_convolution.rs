use proconio::{input, marker::Usize1};
use itertools::Itertools;
use std::collections::HashSet;
const INF: i64 = 1 << 60;
// abc412d: subset convolutioin
// Q. Find the minimum number of operations to make G a simple undirected graph where all vertices have degree 2.
// Key Point: 3 <= N <= 8
// A. Permutation as a Mapping Function = 順列の値とedgeを対応付ける perm[v] = Neighbor of v
// perm[v] = v: Identity Mapping = 恒等写像 or perm[perm[v]] = v
fn main() {
    input!{n: usize, m: usize, ab: [(Usize1, Usize1); m]}
    let mut edges = HashSet::new();
    for &(a, b) in ab.iter() {
        edges.insert((a, b));
        edges.insert((b, a));
    }

    let bits: usize = 1 << n;
    // dp[S] = the overlap of S and Edges
    let mut dp: Vec<i64> = vec![-INF; bits];
    dp[0] = 0;
    // Q. Why n - 2? (Start and End?)
    for start in 0..n-2 {
        let n_rest: usize = n - start;
        // max_overlaps_with_graph[n_rest][S] = 頂点の集合Sに対してサイクルSとG(すでに与えられた辺の集合)の共通部分の最大値
        // ここでの-1はstartを意味している。
        let mut max_overlaps_with_graph: Vec<Vec<i64>> = vec![vec![-INF; (1 << (n_rest - 1))]; n_rest];
        max_overlaps_with_graph[0][0] = 0;
        // s_bit: n_restで指定された頂点の集合の要素としてのbit集合
        // 頂点を一つ決めうっているから-1が必要なのかな?
        for s_bit in 0..(1 << (n_rest - 1)) {
            for v in 0..n_rest {
                if max_overlaps_with_graph[v][s_bit] == -INF {
                    continue;
                }
                // 0: start
                for next_v in 1..n_rest {
                    let next_s_bit: usize = s_bit | (1 << (next_v - 1));
                    if s_bit == next_s_bit {
                        continue;
                    }
                    if edges.contains(&(start+v, start+next_v)) {
                        max_overlaps_with_graph[next_v][next_s_bit] = max_overlaps_with_graph[next_v][next_s_bit].max(max_overlaps_with_graph[v][s_bit] + 1);
                    } else {
                        max_overlaps_with_graph[next_v][next_s_bit] = max_overlaps_with_graph[next_v][next_s_bit].max(max_overlaps_with_graph[v][s_bit]);
                    }
                }
                if v == 0 || s_bit == (1 << (v - 1)) {
                    continue;
                }
                let bit_graph: usize = (s_bit << 1 | 1) << start;
                if edges.contains(&(start+v, start)) {
                    dp[bit_graph] = dp[bit_graph].max(max_overlaps_with_graph[v][s_bit] + 1); 
                } else {
                    dp[bit_graph] = dp[bit_graph].max(max_overlaps_with_graph[v][s_bit]); 
                }
            }
        }
    }

    for s_bit in 0..bits {
        let mut t_bit = s_bit;
        while t_bit > 0 {
            dp[s_bit] = dp[s_bit].max(dp[t_bit] + dp[t_bit ^ s_bit]);
            t_bit = (t_bit - 1) & s_bit;
        }
    }
    let ans = n + m - 2 * dp[bits-1] as usize;
    println!("{}", ans);
}