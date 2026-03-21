use proconio::input;
// ABC410E: Paraphrase of DP
// Q. h: hp, m: mp, ab: (a: hp damage, b: mp damage)
// Key Point: You have no choice but to defeat all monsters. = You cannnot skip a monster!!!
// A. maybe dp? but O(n*h*m) is too large -> Change the target from maximizing kills to minimizing resources used
// A. DP[i][m] を、i 体目のモンスターまで倒した時点で魔力が m であるときの体力の最大値
fn main() {
    input!{n: usize, h: i64, m: usize, ab: [(i64, usize); n]}
    let mut dp_hp: Vec<Vec<i64>> = vec![vec![-1; m + 1]; n + 1];
    dp_hp[0][m] = h;
    for i in 1..=n {
        let hp_damage: i64 = ab[i-1].0;
        let mp_damage: usize = ab[i-1].1;
        for mp in 0..=m {
            // Use HP to defeat the i-th monster
            dp_hp[i][mp] = dp_hp[i][mp].max(dp_hp[i - 1][mp] - hp_damage);
            // Use MP to defeat the i-th monster
            if mp + mp_damage <= m {
                dp_hp[i][mp] = dp_hp[i][mp].max(dp_hp[i - 1][mp + mp_damage]);
            }
        }
    }
    for num_of_kills in (0..=n).rev() {
        for mp in 0..=m {
            // you can defeat nth monster and have non-negative hp
            if dp_hp[num_of_kills][mp] >= 0 {
                println!("{}", num_of_kills);
                return;
            }
        }
    }
}