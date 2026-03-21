use proconio::input;
// abc094D
// nCrが最大になるようにaから数を選ぶ(r=n/2)
// 1. r>0のとき、comb(n+1,r)>comb(n,r) (パスカルの三角形)
// 2. r+1<=n/2のとき、comb(n,r+1) > comb(n,r): rをなるべく大きくする
fn main() {
    input!{n: usize, mut a: [i64; n]}
    a.sort();
    // fixed
    let large = a[n-1];
    let mut small = a[0];
    for i in 0..n-1 {
        // nCrにおいてrがn/2にできるだけ近くなるように選ぶ
        // > ではなく >= を入れる!(rをなるべく大きくする)
        if (large / 2 - small).abs() >= (large / 2 - a[i]).abs() {
            small = a[i];
        }
    }
    println!("{} {}", large, small);
}