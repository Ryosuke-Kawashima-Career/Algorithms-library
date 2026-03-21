use proconio::input;
// abc377d
// Q. 1≤l≤r≤Mに対して，全ての1≤i≤N に対し区間[l,r]は区間[Li,Ri]を完全には含まない。
// A. 整数の組(l,r)が条件を満たすなら,(l+1,r)も条件を満たす
// r を固定した際に(l,r)が条件を満たすようなlはdr以上r以下の整数のみ
// dr−1が求まっていると仮定してdrを求める。
// もし Ri =r を満たすiが存在しない場合、新しく制約は増えないので dr=dr−1。
// そのような iが存在する場合、Ri =r を満たすiに対するLiの最大値をLmaxとすると, dr=max(dr−1,Lmax+1)
// この漸化式にしたがってd1から順に求めていくことでdrを求めることができる
fn main() {
    input!{n: usize, m: usize, lr: [(usize, usize); n]}
    let mut d: Vec<usize> = vec![1; m+1];
    for &(l, r) in lr.iter() {
        d[r] = d[r].max(l + 1);
    }
    for r in 1..=m {
        d[r] = d[r].max(d[r-1]);
    }
    let mut ans: usize = 0;
    for r in 1..=m {
        ans += r + 1 - d[r];
    }
    println!("{}", ans);
}