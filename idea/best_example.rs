use proconio::input;
// abc125d
// 問題の言い換え
// parity
// 不変量
fn main() {
    input!{n: usize, a: [i64; n]}
    // この問題では負符号のparityが不変量(a[i], a[i+1]の符号を反転するから)
    let mut parity: usize = 0;
    let mut min_abs: i64 = 1 << 60;
    // 数列の和の上界を考える。
    let mut upper_sum: i64 = 0;

    for i in 0..n {
        if a[i] < 0 {
            parity += 1;
            parity %= 2;
        }
        upper_sum += a[i].abs();
        min_abs = min_abs.min(a[i].abs());
    }

    let max_sum: i64 = if parity == 0 {
        // parityが0の時、上界は達成される。
        upper_sum
    } else {
        upper_sum - 2*min_abs
    };

    println!("{}", max_sum);
}