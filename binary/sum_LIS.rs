use proconio::input;
use superslice::Ext;
// 鉄則C07
// LISの考え方をsumに応用
fn main() {
    input!{n: usize, mut c: [i64; n], q: usize}
    c.sort();
    // i個買うのに必要な最小の金額 = prefix_min_top[i]
    let mut prefix_min_top: Vec<i64> = vec![0; n+1];
    // 累積和によって求める。
    for i in 1..=n {
        prefix_min_top[i] = prefix_min_top[i-1] + c[i-1];
    }

    for _ in 0..q {
        input!{x: i64}
        // x円でいくつの品物を変えるか: LISの応用
        // x円以下!なのでupperbound
        let length: usize = prefix_min_top.upper_bound(&x);
        println!("{}", length - 1);
    }
}