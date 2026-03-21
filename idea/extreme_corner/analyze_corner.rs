use proconio::input;
// 典型75
// Nの特殊な制約
// N=1などの特殊なケース
// 小さな具体例で実験する
// 「必ず1減る」「操作によって変わらない」など不変量を見つける
// {a, b, c}を{0, 1, 2}などの数字に置き換える
fn main() {
    input!{n: usize}
    let primes: usize = count_primes(n);
    let log_primes = primes.ilog2() as usize;
    let ans: usize = if 2usize.pow(log_primes as u32) == primes {
        // 素因数が2の累乗のとき(corner case)
        log_primes
    } else {
        log_primes + 1
    };
    println!("{}", ans);
}

fn count_primes(num: usize) -> usize {
    let mut n: usize = num;
    let mut div: usize = 2;
    let mut cnt: usize = 0;

    while div * div <= num {
        // 素数divで割り切れるまで計算する
        while n % div == 0 {
            cnt += 1;
            n /= div;
        }
        div += 1;
    }

    // numが素数のとき(corner case)
    if n > 1 {
        cnt += 1;
    }

    return cnt;
}