use proconio::input;
// 最小公倍数が周期になる
fn main() {
    // nとmのどちらか片方で割れるk番目の数
    input!{n: usize, m: usize, k: usize}
    // 0indexedに直す!!!
    let k: usize = k - 1;

    // 周期を求める
    let lcm_nm: usize = lcm(n, m);
    let group_n: usize = lcm_nm / n - 1;
    let group_m: usize = lcm_nm / m - 1;
    let group: usize = group_n + group_m;

    // 周期groupの数と周期の中で何番目か
    let groups: usize = k / group;
    let remain: usize = k % group;

    // nとmの倍数で小さいほうを採用する
    let mut mul_n: usize = 1;
    let mut mul_m: usize = 1;
    let mut cur_num: usize = 0;
    for _ in 0..=remain {
        if n * mul_n < m * mul_m {
            cur_num = n * mul_n;
            mul_n += 1;
        } else {
            cur_num = m * mul_m;
            mul_m += 1;
        }
    }
    
    let ans: usize = cur_num + lcm_nm * groups;
    println!("{}", ans);
}

fn gcd(n: usize, m: usize) -> usize {
    if n == 0 || m == 0 {
        return n + m;
    }

    return gcd(m, n % m);
}

fn lcm(n: usize, m: usize) -> usize {
    n / gcd(n, m) * m
}