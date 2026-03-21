use proconio::{input, marker::Chars};
// abc375D
// Q. 1≤i<j<k≤∣S∣かつSi,Sj,Skをこの順に結合して得られる長さ3の文字列が回文となる
// Ex. ABCACC
// A. 1. 中心の文字に注目する 2. アルファベットの累積和 3. 文字ごとにカウント
fn main() {
    input!{s: Chars}
    let n: usize = s.len();
    // prefix[index][alphabet]
    let mut prefix: Vec<Vec<usize>> = vec![vec![0; 26]; n+1];
    for i in 1..=n {
        for j in 0..26 {
            if j + 'A' as usize == s[i-1] as usize {
                prefix[i][j] = prefix[i-1][j] + 1;
            } else {
                prefix[i][j] = prefix[i-1][j];
            }
        }
    }
    let mut ans: usize = 0;
    for i in 1..=n {
        for j in 0..26 {
            let left = prefix[i-1][j];
            let right = prefix[n][j] - prefix[i][j];
            ans += left * right;
        }
    }
    println!("{}", ans);
}