use proconio::input;
// abc027c
// Q. x -> 2*x or 2*x + 1: Nを超えたら負け(x0 = 1)
// A. いくつか手段があるときは、同じ手段を取り続けるのが最善の時がある
// A. 初手が勝つ範囲: y((z-1)/2)..z(y/2) -> yが先に1以下になれば先手の勝利, z: 敗北
// A. y0 -> z0 -> y1 -> z1 -> ...
fn main() {
    // N = 1..=10^18
    input!{n: usize}
    // 0: 先手, 1: 後手
    let mut player: usize = 0;
    let mut x: usize = n + 1;

    while x > 1 {
        if player == 0 {
            x = (x + 1) / 2;
        } else {
            x = x / 2;
        }
        // 先手と後手を入れ替える
        player ^= 1;
    }

    println!("{}", if player == 0 {"Takahashi"} else {"Aoki"});
}