use proconio::input;
const N: usize = 60;
const MOD: usize = 998244353;
// abc356D
// Q. Σk=0..=n popcount(k & m)
// 桁ごとにpopcount
fn main() {
    input!{n: usize, m: usize}
    let mut ans: usize = 0;
    // 全桁調べる
    for digit in 0..N {
        if m >> digit & 1 == 1 {
            ans += count(digit, n);
            ans %= MOD;
        }
    }
    println!("{}", ans);
}

fn count(digit: usize, limit: usize) -> usize {
    // digitビット目のビットが1となる値の総数の基本パターンを計算。
    // 2^digit が limit 以下の値の中で何回登場するかを計算する。
    let mut res: usize = (limit >> (digit+1)) << digit;

    // limitのdigit ビット目が 1 かどうかで端数を計算
    if limit >> digit & 1 == 1 {
        // 端数を計算
        res += (limit & ((1 << digit) - 1)) + 1;
        // Ex. limit = 10111, digit = 2 の場合、
        // 1 << 2 -> 100
        // 100 - 1 -> 011
        // 10111 & 011 -> 011
        // 100 を数えたいので
        // 011 + 1 -> 100
        // 十進数で 4
    }
    return res;
}