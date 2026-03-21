use proconio::input;
const INF: i64 = 1 << 60;
// abc443D
// Q. Find the minimum number of operations to make all rows connected with the Euclidean distance = 1 among the neighboring columns.
// A. Lower Upper Bound by the Bidirectional Search.
// 左からの値と右からの値でLower Boundを計算する。左右から挟み撃ちにする感じ。
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
    let mut upper_bounds_left: Vec<i64> = vec![INF; n];
    let mut upper_bounds_right: Vec<i64> = vec![INF; n];
    for col in 0..n {
        if col > 0 {
            upper_bounds_left[col] = upper_bounds_left[col].min(upper_bounds_left[col - 1] + 1);
        }
        upper_bounds_left[col] = upper_bounds_left[col].min(rows[col]);
    }
    for col in (0..n).rev() {
        if col < n - 1 {
            upper_bounds_right[col] = upper_bounds_right[col].min(upper_bounds_right[col + 1] + 1);
        }
        upper_bounds_right[col] = upper_bounds_right[col].min(rows[col]);
    }
    let mut cost: i64 = 0;
    for col in 0..n {
        cost += rows[col] - (upper_bounds_left[col].min(upper_bounds_right[col]));
    }
    cost as usize
}
