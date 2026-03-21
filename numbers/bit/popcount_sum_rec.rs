use proconio::input;
const N: usize = 60;
const MOD: usize = 998244353;
// abc356D
// Q. Σk=0..=n popcount(k & m)
// 各桁の寄与率で計算する
// 2^xで二次元的に場合分けする
fn main() {
    input!{n: usize, m: usize}
    let ans = solve(n, m);
    println!("{}", ans);
}

fn solve(n: usize, m: usize) -> usize {
    // base case
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return m & 1;
    }
    let k: usize = msb(n);
    let res1: usize = (m & ((1 << k) - 1)).count_ones() as usize * (1 << (k - 1));
    let res2: usize = if m >> k & 1 == 1 {
        n - ((1 << k) - 1)
    } else {
        0
    };
    let res3: usize = solve(n - (1 << k), m);

    let res = ((res1 % MOD + res2 % MOD) % MOD + (res3 % MOD)) % MOD;
    return res;
}

// most significant bit
fn msb(bit: usize) -> usize {
    for digit in (0..N).rev() {
        if bit >> digit & 1 == 1 {
            return digit;
        }
    }
    return 0;
}