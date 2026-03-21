use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
// abc443D
// Q. Find the minimum number of operations to make all rows connected with the Euclidean distance = 1 among the neighboring columns.
// A. Focus on the top pawn because it is sedentary.
// 一番上の駒を固定して、それらの隣の駒を指定の高さに動かせばいい!
fn main() {
    input! {t: usize}
    let mut answer: Vec<usize> = Vec::new();
    for _case in 0..t {
        let ans = solve();
        answer.push(ans);
    }
    for i in 0..t {
        println!("{}", answer[i]);
    }
}

fn solve() -> usize {
    input! {n: usize, rows: [i64; n]}
    let mut after_rows: Vec<i64> = rows.clone();
    // (row, col)
    let mut top_pawns: BinaryHeap<(Reverse<i64>, usize)> = BinaryHeap::new();
    for i in 0..n {
        top_pawns.push((Reverse(rows[i]), i));
    }
    // Check whether the column has been visited.
    let mut visited: Vec<bool> = vec![false; n];
    while let Some((Reverse(top_row), top_col)) = top_pawns.pop() {
        if visited[top_col] {
            continue;
        }
        visited[top_col] = true;
        if top_col > 0 {
            after_rows[top_col - 1] = (top_row + 1).min(after_rows[top_col - 1]);
            top_pawns.push((Reverse(after_rows[top_col - 1]), top_col - 1));
        }
        if top_col < n - 1 {
            after_rows[top_col + 1] = (top_row + 1).min(after_rows[top_col + 1]);
            top_pawns.push((Reverse(after_rows[top_col + 1]), top_col + 1));
        }
    }
    let mut cost: usize = 0;
    for i in 0..n {
        cost += (rows[i] - after_rows[i]) as usize;
    }
    cost
}
