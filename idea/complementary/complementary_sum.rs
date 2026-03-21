use proconio::input;
// abc346C
// 余事象: sum(1..=k)から要らない要素を消し去る
// 1以上K以下の整数のうち、Aの中に一度も現れないものの総和
fn main() {
    input!{n: usize, k: usize, a: [usize; n]}
    let mut sum: usize = (1 + k) * k / 2;
    // ダブりを防ぐ
    let mut used = std::collections::HashSet::new();
    for i in 0..n {
        if 1 <= a[i] && a[i] <= k && !used.contains(&a[i]) {
            sum -= a[i];
        }
        used.insert(a[i]);
    }
    println!("{}", sum);
}