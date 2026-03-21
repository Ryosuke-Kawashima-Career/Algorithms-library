use proconio::input;
// abc034d
// 濃度piパーセントの食塩水がwiグラムある。
// k個選んで最大の濃度を達成する。
// 答え(濃度)の二分探索
fn main() {
    // w: weight, p: percent
    input!{n: usize, k: usize, wp: [(f64, f64); n]}
    // l: 実現可能, r: 不可能
    let mut l: f64 = 0.0;
    let mut r: f64 = 100.0;
    for _ in 0..100 {
        let mid: f64 = (l + r) / 2.0;
        // 濃度midを達成する塩が十分あるか?
        if is_salt_enough(mid, k, &wp) {
            l = mid;
        } else {
            r = mid;
        }
    }
    println!("{}", l);
}

fn is_salt_enough(conc: f64, k: usize, wp: &Vec<(f64, f64)>) -> bool {
    let mut salts: Vec<f64> = Vec::new();
    for &(weight, percent) in wp.iter() {
        // 濃度の差 * 重量 = 塩の量
        let salt: f64 = weight * (percent - conc);
        salts.push(salt);
    }
    salts.sort_by(|x, y| y.partial_cmp(&x).unwrap());
    let salt_sum: f64 = salts.iter().take(k).sum();
    return salt_sum >= 0.0;
}