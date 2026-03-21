use proconio::{input, marker::Usize1};
use itertools::Itertools;
use std::collections::HashSet;
// abc412d
// Q. Find the minimum number of operations to make G a simple undirected graph where all vertices have degree 2.
// Key Point: 3 <= N <= 8
// 1-4-5, 2-4-6の二つのサイクルがある場合に注意する。一方でN<=8なので、島は2つより多くならない。
// A. Permutation Brute Force
// 制約条件に注目する。
fn main() {
    input!{n: usize, m: usize, ab: [(Usize1, Usize1); m]}
    let mut edges = HashSet::new();
    for &(a, b) in ab.iter() {
        edges.insert((a, b));
        edges.insert((b, a));
    }
    let mut ans: usize = 1 << 60;
    for perm in (0..n).permutations(n) {
        let mut edge_overlaps: usize = 0;
        for i in 0..n {
            if edges.contains(&(perm[i % n], perm[(i+1) % n])) {
                edge_overlaps += 1;
            }
        }
        let cur: usize = n + m - 2 * edge_overlaps;
        ans = ans.min(cur);
    }
    if n >= 6 {
        for perm in (0..n).permutations(n) {
            for n1 in 3..n {
                let mut edge_overlaps: usize = 0;
                let n2: usize = n - n1;
                if n2 < 3 {
                    break;
                }
                for i1 in 0..n1 {
                    if edges.contains(&(perm[i1 % n1], perm[(i1+1) % n1])) {
                        edge_overlaps += 1;
                    }
                }
                for i2 in 0..n2 {
                    if edges.contains(&(perm[(i2 % n2) + n1], perm[(i2 + 1) % n2 + n1])) {
                        edge_overlaps += 1;
                    }
                }
                let cur: usize = n + m - 2 * edge_overlaps;
                ans = ans.min(cur);
            }
        }
    }
    println!("{}", ans);
}