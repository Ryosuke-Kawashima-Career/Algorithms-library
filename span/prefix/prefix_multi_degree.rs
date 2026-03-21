use proconio::input;
// abc423e
// Q. 各Ajに対し、式(l=Li∑Ri)(r=l∑Ri)(j=l∑r)AjでAj が足される回数
// A. 左のΣから変数を確定する。-> j^2Aj,jAj,Ajの累積和を持っておき、区間和を取ったのちに係数を掛け合わせて和を取ればよい
fn main() {
    input!{n: usize, q: usize, a: [i64; n], lr: [(usize, usize); q]}
    let mut prefix_aj: Vec<i64> = vec![0; n+1];
    let mut prefix_jaj: Vec<i64> = vec![0; n+1];
    let mut prefix_j2aj: Vec<i64> = vec![0; n+1];
    for j in 1..=n {
        prefix_aj[j] = prefix_aj[j-1] + a[j-1];
        prefix_jaj[j] = prefix_jaj[j-1] + (j as i64) * a[j-1];
        prefix_j2aj[j] = prefix_j2aj[j-1] + (j as i64) * (j as i64) * a[j-1];
    }

    for &(l, r) in lr.iter() {
        let ans: i64 = - (prefix_j2aj[r] - prefix_j2aj[l-1]) + (l + r) as i64 * (prefix_jaj[r] - prefix_jaj[l-1]) + - (l as i64 - 1) * (r as i64 + 1) * (prefix_aj[r] - prefix_aj[l-1]);
        println!("{}", ans);
    }
}