use proconio::input;
// abc098D: 尺取り法([l r]の区間を扱う)
// 単調増加性に着目する
// Q. Al xor Al+1 xor ... xor Ar = Al + Al+1 + ... + Ar となる(l, r)の組
// A. sum + a[i] == sum ^ a[i] (数学的帰納法のようなドミノ倒し)
fn main() {
    input!{n: usize, a: [usize; n]}
    let mut ans: usize = 0;
    // 尺取りの左側
    let mut cur_index: usize = 0;
    // 条件判定用の変数(rに対して単調増加する！)
    let mut sum: usize = 0;
    for i in 0..n {
        let mut index: usize = cur_index;
        // 条件をギリギリ満たさないときに止める
        while index < n && sum + a[index] == sum ^ a[index] {
            sum += a[index];
            index += 1;
        }

        ans += index - i;
        // もとに戻す
        sum -= a[i];
        // 尺取りの左側を動かす
        if index == i {
            cur_index = i + 1;
        } else {
            cur_index = index;
        }
    }

    println!("{}", ans);
}