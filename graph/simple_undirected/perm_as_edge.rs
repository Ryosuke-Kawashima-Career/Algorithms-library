use proconio::{input, marker::Usize1};
use itertools::Itertools;
use std::collections::HashSet;
// abc412d
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
    let mut ans: usize = 1 << 60;
    for perm in (0..n).permutations(n) {
        let mut over_laps: usize = 0;
        // check whether all the degrees are 2.
        let mut degrees: Vec<usize> = vec![0; n];
        for v in 0..n {
            degrees[v] += 1;
            degrees[perm[v]] += 1;
            if edges.contains(&(v, perm[v])) {
                over_laps += 1;
            }
        }
        if is_simple_undirected(&perm) && check_degrees(&degrees) {
            let candidate: usize = n + m - 2 * over_laps;
            ans = ans.min(candidate);
        }
    }
    println!("{}", ans);
}

fn is_simple_undirected(perm: &Vec<usize>) -> bool {
    let n: usize = perm.len();
    for v in 0..n {
        if perm[v] == v || perm[perm[v]] == v {
            return false;
        }
    }
    return true;
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