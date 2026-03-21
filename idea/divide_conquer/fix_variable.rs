use proconio::input;
// abc099c(DPもOK: n円を6^x, 9^x円の硬貨で作るときの最小の枚数 = dp[n])
// 一気に2つ以上のことをしようとしない
// 分解して考える（シンプルな場合を考える）
// 一方を固定する
// ex1. 2次元の問題などは、実は1次元に分解して独立して考えることができる
// ex2. 入力が「整数の範囲」なら、「正の数だけの場合」「負の数だけの場合」「0だけの場合」
// ex3. 「1, 2, 3 しか登場しない」なら、「2 と 3 だけの場合」
fn main() {
    input!{n: usize}
    // 6^x 円の形であらわされる金額」のみでいくら引き出すかを全通り試す
    let mut min_coins: usize = n;
    
    // 6^xを固定する
    // 6^x円を6枚以上使う必要はない(6^(x+1)が上位互換)
    // 6^x円はfloor(A÷6^x)%6回引き出せばいい
    for price in 0..=n {
        let mut cur_coins: usize = 0;
        let mut sum6 = price;
        // 値が小さい硬貨から計算する
        while sum6 > 0 {
            cur_coins += sum6 % 6;
            sum6 /= 6;
        }

        let mut sum9 = n - price;
        while sum9 > 0 {
            cur_coins += sum9 % 9;
            sum9 /= 9;
        }

        min_coins = min_coins.min(cur_coins);
    }

    println!("{}", min_coins);
}