use proconio::input;
use superslice::Ext;
// 典型60
const INF: i64 = 1 << 60;

// 両端から最長増加部分列を考える。
fn main() {
    input!{n: usize, a: [i64; n]}
    // 長さがiのLISの最終要素の最小値 = lis_mintop[i: 最長増加部分列の長さ]
    // dpのテーブル
    let mut lis_mintop: Vec<i64> = vec![INF; n+1];
    let mut lis_mintop_rev: Vec<i64> = vec![INF; n+1];
    // とりうる値の最小値より小さいものを用意(a[i] >= 0の時、最小値を-1に設定する)
    lis_mintop[0] = 0;
    lis_mintop_rev[0] = 0;
    // a[i]ごとの最長増加部分列の大きさ
    let mut length: Vec<usize> = vec![0; n];
    let mut length_rev: Vec<usize> = vec![0; n];

    // 左から見たLIS
    for i in 0..n {
        // indexがa[i]に対する最長増加部分列の長さ
        let index: usize = lis_mintop.lower_bound(&a[i]);
        lis_mintop[index] = a[i];
        length[i] = index;
    }
    // 右から見たLIS
    for i in (0..n).rev() {
        let index_rev: usize = lis_mintop_rev.lower_bound(&a[i]);
        lis_mintop_rev[index_rev] = a[i];
        length_rev[i] = index_rev;
    }

    let mut ans: usize = 0;
    for i in 0..n {
        let length_lis: usize = length[i] + length_rev[i] - 1;
        ans = ans.max(length_lis);
    }

    println!("{}", ans);
}