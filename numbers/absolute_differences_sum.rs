use proconio::input;
const MOD: i64 = 998244353;
// ABC437D
// Q. Find the value of i=1∑N j=1∑M ∣Ai−Bj∣
// A. prefix + Binary Search => The Spanning Sum of Absolute Values
// 区間和なので累積和を使うんだろうなとは思った。
// 絶対値を外すときは-1をかければいい。
fn main() {
    input! {n: usize, m: usize, mut a: [i64; n], b: [i64; m]}
    a.sort();
    let mut prefix_a: Vec<i64> = vec![0; n + 1];
    for i in 1..=n {
        prefix_a[i] = prefix_a[i - 1] + a[i - 1];
    }
    let mut ans: i64 = 0;
    for i in 0..m {
        let index: usize = a.partition_point(|&x| x <= b[i]);
        let mut cnt: i64 = 0;
        // b[i]より小さいaの区間和
        let sum_l: i64 = prefix_a[index];
        // b[i]より大きいaの区間和
        let sum_r: i64 = prefix_a[n] - prefix_a[index];
        cnt += sum_r - sum_l;
        // 絶対値を外した時にb[i]が全体の値にどのように影響を与えるのかを考える。
        cnt += index as i64 * b[i];
        cnt -= (n - index) as i64 * b[i];
        ans += cnt;
        if ans < 0 {
            ans += MOD;
        }
        ans %= MOD;
    }
    if ans < 0 {
        ans += MOD;
    }
    println!("{}", ans % MOD);
}
