use proconio::input;
use superslice::Ext;
// abc077c: snuke festival
// 探索する対象を減らす: 真ん中が決まればすべて決まる
fn main() {
    input!{n: usize, mut a: [usize; n], b: [usize; n], mut c: [usize; n]}
    a.sort();
    c.sort();
    let mut ans: usize = 0;
    for i in 0..n {
        // a < b < c
        // 未満の数: lower, 以下の数: upper
        // b[i]未満の個数
        let a_num: usize = a.lower_bound(&b[i]);
        // b[i]より大きい個数
        let c_num: usize = n - c.upper_bound(&b[i]);
        ans += a_num * c_num;
    }
    println!("{}", ans);
}