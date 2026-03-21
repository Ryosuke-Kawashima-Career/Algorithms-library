use proconio::input;
// abc125c
// 要素を一つ除くprefix: 左右から攻める
// 整数を1つ選んで、1以上10^9以下の好きな整数に書き換え, 最大公約数を最大化する
fn main() {
    input!{n: usize, a: [usize; n]}
    // 左右から最大公約数を計算する
    let mut prefix_left: Vec<usize> = vec![0; n+2];
    let mut prefix_right: Vec<usize> = vec![0; n+2];
    prefix_left[0] = a[0];
    prefix_right[n+1] = a[n-1];
    for i in 1..=n {
        prefix_left[i] = gcd(prefix_left[i-1], a[i-1]);
        prefix_right[n+1-i] = gcd(prefix_right[n+2-i], a[n-i]);
    }

    let mut max_gcd: usize = 1;
    for i in 1..=n {
        // corner case
        let gcd_cur = if i == 1 {
            prefix_right[i+1]
        } else if i == n {
            prefix_left[i-1]
        } else {
            gcd(prefix_left[i-1], prefix_right[i+1])
        };
        max_gcd = max_gcd.max(gcd_cur);
    }
    println!("{}", max_gcd);
}

fn gcd(m: usize, n: usize) -> usize {
    if m == 0 || n == 0 {
        return m + n;
    }
    gcd(n, m % n)
}