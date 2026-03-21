use proconio::input;
use superslice::Ext;
const INF: i64 = 1 << 60;
// abc134e
// 後ろから考える!!!(貪欲法)
// 与えられた数列の、広義単調減少列の長さの最大値L
// 数列を反転して、広義増加部分列を計算する
fn main() {
    input!{n: usize, mut a: [i64; n]}
    // 反転する
    a.reverse();

    let mut min_top: Vec<i64> = vec![INF; n+1];
    min_top[0] = -1;
    let mut max_length: usize = 0;

    for i in 0..n {
        // 広義: upper_bound
        let length: usize = min_top.upper_bound(&a[i]);
        max_length = max_length.max(length);
        if length <= n {
            min_top[length] = a[i];
        }
    }
    println!("{}", max_length);
}