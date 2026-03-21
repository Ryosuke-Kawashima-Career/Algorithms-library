use proconio::input;
// abc419C
// Q. Get the minimum time for everyone to get to the same point.
// A. Tails, You win! Only Max and Min matter.
fn main() {
    input!{n: usize, rc: [(i64, i64); n]}
    let mut rows: Vec<i64> = Vec::new();
    let mut cols: Vec<i64> = Vec::new();
    for &(r, c) in rc.iter() {
        rows.push(r);
        cols.push(c);
    }
    rows.sort();
    cols.sort();
    let diff_row: i64 = rows[n-1] - rows[0];
    let diff_col: i64 = cols[n-1] - cols[0];
    // Ceiling Division
    let time_row: i64 = (diff_row + 1) / 2;
    let time_col: i64 = (diff_col + 1) / 2;
    let ans = time_row.max(time_col);
    println!("{}", ans);
}