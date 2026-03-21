use proconio::input;
// ABC410E: Paraphrase of DP
// Q. h: hp, m: mp, ab: (a: hp damage, b: mp damage)
// Key Point: You have no choice but to defeat all monsters. = You cannnot skip a monster!!!
// A. maybe dp? but O(n*h*m) is too large -> Change the target from maximizing kills to minimizing resources used
// A. dp[h][m]:=( 体力が h 、魔力が m のときに倒せるモンスターの数の最大値 )
// Distribution Version is easier to implement.
fn main() {
    input! {n: usize, h: usize, m: usize, ab: [(usize, usize); n]}
    // 倒した数から0..nの内どのiを選べばいいかがわかる!!!
    let mut dp: Vec<Vec<usize>> = vec![vec![0; m + 1]; h + 1];
    
    for hp in 0..=h {
        for mp in 0..=m {
            // targetをそろえたいので、配る方のDPの方が実装しやすい!!!
            // Get current number of defeated monsters: あらかじめ別の変数に保存する。
            let t = dp[hp][mp];
            
            // Propagate current value upward
            // the higer the HP and MP are, the better => 上位互換
            if hp + 1 <= h {
                dp[hp + 1][mp] = dp[hp + 1][mp].max(t);
            }
            if mp + 1 <= m {
                dp[hp][mp + 1] = dp[hp][mp + 1].max(t);
            }
            
            // Try to defeat next monster
            if t < n {
                let (hp_damage, mp_damage) = ab[t];
                if hp + hp_damage <= h {
                    dp[hp + hp_damage][mp] = dp[hp + hp_damage][mp].max(t + 1);
                }
                if mp + mp_damage <= m {
                    dp[hp][mp + mp_damage] = dp[hp][mp + mp_damage].max(t + 1);
                }
            }
        }
    }
    
    println!("{}", dp[h][m]);
}