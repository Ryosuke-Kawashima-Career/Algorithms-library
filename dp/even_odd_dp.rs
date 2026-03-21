use proconio::input;
const INF: i64 = 1_000_000_007;
// abc369d
// Q. 今まで倒したモンスターの数が偶数の時2*a[i], 奇数の時はa[i]の得点を得る
// A. 偶数と奇数のDPを構成する
fn main() {
    input!{n: usize, a: [i64; n]}
    let mut dp_even: Vec<i64> = vec![-INF; n+1];
    let mut dp_odd: Vec<i64> = vec![-INF; n+1];
    // init
    dp_even[0] = 0;
    for i in 1..=n {
        // do nothing
        dp_even[i] = dp_even[i].max(dp_even[i-1]);
        dp_odd[i] = dp_odd[i].max(dp_odd[i-1]);
        // kill a monster
        dp_even[i] = dp_even[i].max(dp_odd[i-1] + 2 * a[i-1]);
        dp_odd[i] = dp_odd[i].max(dp_even[i-1] + a[i-1]);
    }
    let ans = dp_even[n].max(dp_odd[n]);
    println!("{}", ans);
}