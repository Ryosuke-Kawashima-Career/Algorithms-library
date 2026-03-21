use proconio::input;
const MOD: usize = 100;
// 鉄則B40
// categorize by mod
fn main() {
    input!{n: usize, a: [usize; n]}
    let mut cnt_mod: Vec<usize> = vec![0; MOD];
    for i in 0..n {
        cnt_mod[a[i]%MOD] += 1;
    }

    let mut ans: usize = 0;
    // combination of MOD
    for mod1 in 0..MOD {
        for mod2 in mod1..MOD {
            // 和がmodの倍数になるか判定する
            if (mod1 + mod2) % MOD == 0 {
                if mod1 == mod2 {
                    if cnt_mod[mod1] > 1 {
                        ans += cnt_mod[mod1] * (cnt_mod[mod1] - 1) / 2;
                    }
                } else {
                    ans += cnt_mod[mod1] * cnt_mod[mod2];
                }
            }
        }
    }
    println!("{}", ans);
}