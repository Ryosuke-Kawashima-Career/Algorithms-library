use proconio::{input, marker::Chars};
const MOD: usize = 998244353;
// abc312D
// 漸化式を立てDP: 前の状況が後に影響を及ぼす
// (, )からなる文字列Sが括弧列であることは以下の2つの条件を満たすことと同値
// 1. 任意のiについて、Sのi 文字目までに含まれる ( の個数は ) の個数以上
// 2. S に含まれる ( の個数と ) の個数は等しい
fn main() {
    input!{s: Chars}
    let n: usize = s.len();
    // dp[i: item][j: (の数 ※)はi-j個]
    // i-jがjを超えてはいけない
    let mut dp: Vec<Vec<usize>> = vec![vec![0; n+1]; n+1];
    dp[0][0] = 1;
    for i in 1..=n {
        for j in 0..=n {
            match s[i-1] {
                '(' => choose_left(i, j, &mut dp),
                ')' => choose_right(i, j, &mut dp),
                '?' => {
                    choose_left(i, j, &mut dp);
                    choose_right(i, j, &mut dp);
                },
                _ => println!("Input Error!"),
            }
        }
    }

    // カギ括弧の列の長さは必ず偶数
    if n % 2 == 0 {
        println!("{}", dp[n][n/2]);
    } else {
        println!("{}", 0);
    }
}

fn choose_left(index1: usize, num_of_left: usize, dp: &mut Vec<Vec<usize>>) {
    if index1 == 0 || num_of_left == 0 {
        return;
    }
    let prev_left = num_of_left-1;
    let prev_index1 = index1-1;
    let prev_right = prev_index1 - prev_left;
    if prev_left >= prev_right {
        let prev = dp[prev_index1][prev_left];
        dp[index1][num_of_left] += prev;
        dp[index1][num_of_left] %= MOD;
    }
}
fn choose_right(index1: usize, num_of_left: usize, dp: &mut Vec<Vec<usize>>) {
    if index1 == 0 || num_of_left == 0 {
        return;
    }
    let prev_left = num_of_left;
    let prev_index1 = index1-1;
    let prev_right = prev_index1 - prev_left;
    if prev_left >= prev_right {
        let prev = dp[prev_index1][prev_left];
        dp[index1][num_of_left] += prev;
        dp[index1][num_of_left] %= MOD;
    }
}