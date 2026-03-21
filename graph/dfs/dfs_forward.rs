use proconio::{input, marker::Usize1};
use std::collections::HashMap;
// abc448d
// Q. judge the existence of a path between two nodes with the same label
// A. Depth First Search (Forward and Restoring on backwarding)
fn main() {
    input! {n: usize, a: [i64; n], uv: [(Usize1, Usize1); n-1]}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut visited: Vec<bool> = vec![false; n];
    let mut is_duplicated: Vec<bool> = vec![false; n];
    let mut count: HashMap<i64, usize> = HashMap::new();
    dfs(
        0,
        &mut visited,
        &mut is_duplicated,
        &graph,
        &a,
        &mut count,
        0,
    );
    for v in 0..n {
        if is_duplicated[v] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn dfs(
    v: usize,
    visited: &mut Vec<bool>,
    is_duplicated: &mut Vec<bool>,
    graph: &Vec<Vec<usize>>,
    labels: &Vec<i64>,
    count: &mut HashMap<i64, usize>,
    mut num_duplicates: usize,
) {
    /* count and num_duplicates are the variables that should be restored when going backward */
    if visited[v] {
        return;
    }
    visited[v] = true;
    // Forward!!!
    let mut duplication = false;
    *count.entry(labels[v]).or_insert(0) += 1;
    if *count.get(&labels[v]).unwrap() == 2 {
        duplication = true;
        num_duplicates += 1;
    }
    if num_duplicates > 0 {
        is_duplicated[v] = true;
    }
    for &next in graph[v].iter() {
        if visited[next] {
            continue;
        }
        if duplication {
            is_duplicated[next] = true;
        }
        dfs(
            next,
            visited,
            is_duplicated,
            graph,
            labels,
            count,
            num_duplicates,
        );
    }
    *count.get_mut(&labels[v]).unwrap() -= 1;
    if *count.get(&labels[v]).unwrap() == 1 {
        num_duplicates -= 1;
    }
}

fn dfs_parent_tracking(
    v: usize,
    p: usize,                      // track parent to avoid going backwards
    mut current_duplicates: usize, // count of currently duplicated values on the path
    counts: &mut std::collections::HashMap<i64, usize>,
    is_duplicated: &mut Vec<bool>,
    graph: &Vec<Vec<usize>>,
    labels: &Vec<i64>,
) {
    // 1. Add current vertex's label to the path counter
    let label = labels[v];
    let count = counts.entry(label).or_insert(0);
    *count += 1;

    // 2. If the count becomes 2, we just created a new duplicate pair on our current path!
    if *count == 2 {
        current_duplicates += 1;
    }

    // 3. A path has duplicate values if `current_duplicates > 0`
    if current_duplicates > 0 {
        is_duplicated[v] = true;
    }

    // 4. Traverse children
    for &next in graph[v].iter() {
        if next == p {
            continue; // don't go back to the parent
        }
        dfs(
            next,
            v,
            current_duplicates,
            counts,
            is_duplicated,
            graph,
            labels,
        );
    }

    // 5. Backtrack: Remove current vertex's label as we backtrack up the tree
    let count = counts.get_mut(&label).unwrap();
    if *count == 2 {
        // If it drops from 2 back to 1, we lose a duplicate pair
        current_duplicates -= 1;
    }
    *count -= 1;
}
