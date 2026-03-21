use proconio::input;
use itertools::Itertools;
// abc139d
// 小さい数で実験する
// 上界を見積もる:　上界 = 目的関数 (最大化したいもの) は
// (x1 mod 1) + (x2 mod 2) + ... + (xN mod N) の最大値 =
// (1 mod2)+(2 mod3)+...+((N-1) modN)+(N mod1) = 1+2+...+N-1+0
fn main() {
    input!{n: usize}
    // experiment(n);
    println!("{}", n * (n - 1) / 2);
}

// n = 6 とすると答えが n * (n - 1) / 2
fn experiment(n: usize) {
    for num in 1..=n {
        let mut max_mod_sum: usize = 0;
        for perm in (1..=num).permutations(num) {
            let mut mod_sum: usize = 0;
            for i in 1..=num {
                mod_sum += i % perm[i-1];
            }
            max_mod_sum = max_mod_sum.max(mod_sum);
        }
        println!("{}: {}", num, max_mod_sum);
    }
}