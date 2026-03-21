use proconio::input;
use std::collections::HashMap;
const MOD: usize = 1_000_000_007;
// 鉄則C13
// Q. 相異なるカードを2枚選ぶ方法のうち、次の条件を満たすものはいくつありますか。
// 条件：2枚のカードの積を1000000007で割った余りがPになる。
// A. Modでカードを分類(hashmap) + aのmod上の逆元はpow(a, mod-2)
fn main() {
    input!{n: usize, p: usize, a: [usize; n]}
    let mut map = HashMap::new();
    for i in 0..n {
        *map.entry(a[i] % MOD).or_insert(0) += 1;
    }
    let mut ans: usize = 0;
    // 余りの積が0の時は例外
    if p == 0 {
        if let Some(val0) = map.get(&0) {
            ans += val0 * (n - val0);
        }
    }
    for (&mod1, &val1) in map.iter() {
        let mod2: usize = p * pow(mod1, MOD-2) % MOD;
        if mod1 == mod2 && val1 > 1 {
            ans += val1 * (val1-1) / 2;
        }
        if mod1 < mod2 {
            if let Some(val2) = map.get(&mod2) {
                ans += val1 * val2;
            }
        }
    }
    println!("{}", ans);
}

fn pow(m: usize, n: usize) -> usize {
    if n == 0 {
        return 1;
    }

    let m2: usize = (m * m) % MOD;
    let res: usize = if n % 2 == 0 {
        pow(m2, n / 2)
    } else {
        m * (pow(m2, (n-1) / 2) % MOD)
    };
    res % MOD
}