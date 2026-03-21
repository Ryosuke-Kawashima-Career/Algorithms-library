use proconio::input;
// abc354E
// Q. 二枚のカードで表同士，裏同士の値が同じなら捨てる．この操作を行えなくなったら負け．
// A. N=18: bit全探索? -> bitDP!
fn main() {
    input!{n: usize, ab: [(usize, usize); n]}
    let bits: usize = 1 << n;
    // 自分が勝つ(1: true), 後手(0: false) = dp[bit]
    let mut win_dp: Vec<bool> = vec![false; bits];

    for bit in 0..bits {
        for i in 0..n {
            for j in i+1..n {
                // i, jが選ばれるとき
                if bit >> i & 1 == 1 && bit >> j & 1 == 1 {
                    // 表同士もしくは裏同士が同じ
                    if ab[i].0 == ab[j].0 || ab[i].1 == ab[j].1 {
                        // 相手が負ける遷移があるとき(xorでbitのi,jを0にする)
                        if !win_dp[bit ^ (1 << i) ^ (1 << j)] {
                            win_dp[bit] = true;
                        }
                    }
                }
            }
        }
    }

    if win_dp[bits-1] {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}