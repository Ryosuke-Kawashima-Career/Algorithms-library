use proconio::{input, marker::Chars};
// ABC418D: String Operation (xNor)
// Q. Change the given string to "1"
// A. Conservation of Parity
// この手の文字列や数列に対して操作を行う問題に取り組む場合は、不変量 (操作の前後で変わらない何らかの値) を探すアプローチが有効になる
// 00 を 1 に変化させると、0 の個数は 2 個減る。
// 01 を 0 に変化させると、0 の個数は変わらない。
// 10 を 0 に変化させると、0 の個数は変わらない。
// 11 を 1 に変化させると、0 の個数は変わらない。
// 操作によって0の個数は変わらないか 2 個減る。
// つまり、0の個数の偶奇は不変である!!!
fn main() {
    input!{n: usize, t: Chars}
    // Number of substrings = dp[index][Parity = 0: Even, 1: Odd]
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 2]; n+1];
    for i in 1..=n {
        if t[i-1] == '0' {
            dp[i][0] += dp[i-1][1];
            dp[i][1] += dp[i-1][0] + 1;
        } else {
            dp[i][0] += dp[i-1][0] + 1;
            dp[i][1] += dp[i-1][1];
        }
    }
    let mut count_substrings: usize = 0;
    for end in 1..=n {
        count_substrings += dp[end][0];
    }
    println!("{}", count_substrings);
}