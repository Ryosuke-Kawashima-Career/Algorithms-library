use proconio::{input, marker::Chars};
// abc360d
// Q.数直線上に点がある．すべて速度1で進む．t秒内にいくつの点の組み合わせがすれ違うか？
// A. 正負(A: 正，B: 負)で場合分けして尺取り法
// あるiに対してBj>Aiなるjの最小値はiが増加するにつれて単調に増加
// 同時にBj−Ai≤2×Tを満たすjの最大値もiが増加するにつれて単調に増加
fn main() {
    input!{n: usize, t: i64, s: Chars, x: [i64; n]}
    let mut x_right: Vec<i64> = Vec::new();
    let mut x_left: Vec<i64> = Vec::new();
    for i in 0..n {
        if s[i] == '0' {
            x_left.push(x[i]);
        } else {
            x_right.push(x[i]);
        }
    }
    // 尺取り法を左右で行う
    x_left.sort();
    x_right.sort();
    let mut ans: usize = 0;
    let mut l: usize = 0;
    let mut r: usize = 0;
    for i in 0..x_right.len() {
        // x_right <= x_left <= x_right + 2 * t
        while l < x_left.len() && x_left[l] < x_right[i] {
            l += 1;
        }
        while r < x_left.len() && x_left[r] <= x_right[i] + 2 * t {
            r += 1;
        }
        ans += r - l;
    }
    println!("{}", ans);
}