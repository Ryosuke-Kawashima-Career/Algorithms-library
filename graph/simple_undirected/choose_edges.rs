use proconio::{input, marker::Usize1};
use itertools::Itertools;
use std::collections::HashSet;
// abc412d
// Q. Find the minimum number of operations to make G a simple undirected graph where all vertices have degree 2.
// Key Point: 3 <= N <= 8
// A. Edges are the target of choice
// 発想を逆転させてnodeでなくedgeを選ぶ
fn main() {
    input!{n: usize, m: usize, ab: [(Usize1, Usize1); m]}
    let mut edges = HashSet::new();
    for &(a, b) in ab.iter() {
        edges.insert((a, b));
        edges.insert((b, a));
    }
    // enumerate all the edges in the graph
    let mut all_edges: Vec<(usize, usize)> = Vec::new();
    for u in 0..n {
        for v in u+1..n {
            all_edges.push((u, v));
        }
    }

    let mut ans: usize = 1 << 60;
    // choose n-edges of the candidates of undirected graphs with the degree of 2.
    for comb in (0..all_edges.len()).combinations(n) {
        let mut over_laps: usize = 0;
        // check whether all the degrees are 2.
        let mut degrees: Vec<usize> = vec![0; n];
        for &edge_index in comb.iter() {
            let chosen_edge: (usize, usize) = all_edges[edge_index];
            if edges.contains(&chosen_edge) {
                over_laps += 1;
            }
            degrees[chosen_edge.0] += 1;
            degrees[chosen_edge.1] += 1;
        }
        if check_degrees(&degrees) {
            let candidate: usize = n + m - 2 * over_laps;
            ans = ans.min(candidate);
        }
    }
    println!("{}", ans);
}

fn check_degrees(degrees: &Vec<usize>) -> bool {
    let mut all_2 = true;
    for &degree in degrees.iter() {
        if degree != 2 {
            all_2 = false;
        }
    }
    return all_2;
}