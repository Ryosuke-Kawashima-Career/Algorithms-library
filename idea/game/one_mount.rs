use proconio::input;
// 典型A32
// 石の山のゲーム
// a個かb個、取れる。
fn main() {
    input!{n: usize, a: usize, b: usize}
    // 石がi個あるとき先手が勝つか = win_dp[i]
    let mut win_dp: Vec<bool> = vec![false; n+1];

    // 先手の視点で考える。
    for i in 1..=n {
        // 石をa個とるとき、相手が負ける。
        if i >= a && !win_dp[i-a] {
            win_dp[i] = true;
        }
        // 石をb個とるとき、相手が負ける。
        if i >= b && !win_dp[i-b] {
            win_dp[i] = true;
        }
    }

    // 石の山がn個の石でできているとき
    if win_dp[n] {
        println!("First");
    } else {
        println!("Second");
    }
}