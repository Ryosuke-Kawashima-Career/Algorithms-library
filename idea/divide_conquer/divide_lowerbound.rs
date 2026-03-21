use proconio::input;
const MOD: usize = 100_000_000;
// abc353C: 場合分け
// f(x, y) = (x + y) % MOD
// Q. Σi=0..n-1 Σj=i+1..n (f(Ai, Aj))
// A. 1. x+y < MOD 2. x+y >= MOD
fn main() {
    input!{n: usize, mut a: [usize; n]}
    a.sort();
    let mut prefix: Vec<usize> = vec![0; n+1];
    for i in 1..=n {
        prefix[i] = prefix[i-1] + a[i-1];
    }

    let mut ans: usize = 0;
    for i in 0..n-1 {
        let remain = n - i - 1;
        let index: usize = a[i+1..].partition_point(|&x| x < MOD - a[i]);
        let less = a[i] * index + prefix[i+1 + index] - prefix[i+1];
        let mut greater: usize = a[i] * (remain - index) + prefix[n] - prefix[i+1 + index];
        // MODの寄与分を消す．
        greater -= (remain - index) * MOD;
        ans += less + greater;
    }
    println!("{}", ans);
}