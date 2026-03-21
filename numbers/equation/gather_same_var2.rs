use proconio::input;
use std::collections::HashMap;
// ABC417C
// Q. j−i=Ai+Ajを変形して、j−Aj=i+Ai (i < j の組み合わせを逐次更新で計算する)
// A. Gather the same variable in the same equation
// A. jを1つ固定して条件を満たす(i,j)の組をすべて見つける (Combinations)
fn main() {
    input!{n: usize, a: [isize; n]}
    // counter[x] : x = i + A[i] を満たす i の個数
    let mut counter = HashMap::new();
    let mut ans: usize = 0;

    for i in 0..n {
        // j - A[j] = i + A[i] を満たす個数を加算して
        let key_left: isize = i as isize - a[i];
        if let Some(&num) = counter.get(&key_left) {
            ans += num;
        }
        // j + A[j] のカウンタを進める
        let key_right: isize = i as isize + a[i];
        *counter.entry(key_right).or_insert(0) += 1;
    }
    println!("{}", ans);
}