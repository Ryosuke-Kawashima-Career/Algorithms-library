use proconio::input;
use std::collections::HashMap;
// ABC417C
// Q. j−i=Ai+Ajを変形して、j−Aj=i+Ai (i < j の組み合わせを逐次更新で計算する)
// A. Gather the same variable in the same equation
// A. jを1つ固定して条件を満たす(i,j)の組をすべて見つける (Combinations)
fn main() {
    input!{n: usize, a: [isize; n]}
    // counter[x] : x = j - A[j] を満たす j の個数
    let mut counter = HashMap::new();
    let mut ans: usize = 0;

    for j in 0..n {
        // j - A[j]の個数を加算
        let key_left: isize = j as isize - a[j];
        *counter.entry(key_left).or_insert(0) += 1;
    }

    for i in 0..n {
        let key_right: isize = i as isize + a[i];
        if let Some(&num) = counter.get(&key_right) {
            ans += num;
        }
    }
    println!("{}", ans);
}