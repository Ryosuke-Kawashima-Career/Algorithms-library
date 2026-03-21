use proconio::{input, marker::Usize1};
// abc355C: 値が更新される際に最大値を名乗る挑戦権が与えられる
// ビンゴ(0..n^2)の判定
fn main() {
    // a: マス目の数
    input!{n: usize, t: usize, a: [Usize1; t]}
    let mut used_row: Vec<usize> = vec![0; n];
    let mut used_col: Vec<usize> = vec![0; n];
    let mut used_left: usize = 0;
    let mut used_right: usize = 0;

    for i in 0..t {
        let row: usize = a[i] / n;
        let col: usize = a[i] % n;
        used_row[row] += 1;
        used_col[col] += 1;
        if row == col {
            used_left += 1;
        }
        if row + col == n-1 {
            used_right += 1;
        }

        // 更新したところのみを判定する
        if used_row[row] == n || used_col[col] == n || used_left == n || used_right == n {
            println!("{}", i+1);
            return;
        }
    }

    println!("-1");
}