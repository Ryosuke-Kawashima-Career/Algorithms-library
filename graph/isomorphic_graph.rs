use proconio::{input, marker::Usize1};
use itertools::Itertools;
use std::collections::HashSet;
// abc371C: make Isomorphic
// permutation brute force
fn main() {
    input!{n: usize, 
        mg: usize, edges_g: [(Usize1, Usize1); mg],
        mh: usize, edges_h: [(Usize1, Usize1); mh],
    }
    let mut a: Vec<Vec<usize>> = vec![vec![0; n]; n];
    for i in 0..n {
        for j in i+1..n {
            input!{aij: usize}
            a[i][j] = aij;
            a[j][i] = aij;
        }
    }
    let mut edges_set_g = HashSet::new();
    let mut edges_set_h = HashSet::new();
    for &(x, y) in edges_g.iter() {
        edges_set_g.insert((x, y));
        edges_set_g.insert((y, x));
    }
    for &(x, y) in edges_h.iter() {
        edges_set_h.insert((x, y));
        edges_set_h.insert((y, x));
    }

    let mut ans: usize = 1 << 60;
    // H's vertexes (i) corresponds to G's vertexes (perm[i])
    for perm in (0..n).permutations(n) {
        let mut cur: usize = 0;
        // Check edge (i -> j)
        for i in 0..n {
            for j in i+1..n {
                // if Graph G and Graph H are different 
                if edges_set_g.contains(&(perm[i], perm[j])) 
                != edges_set_h.contains(&(i, j)) 
                {
                    cur += a[i][j];
                }
            }
        }
        ans = ans.min(cur);
    }
    println!("{}", ans);
}