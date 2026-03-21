use proconio::input;
// arc054b
// 探索の対象の関数が凸関数: 三分探索
// 三分探索: たかだか一つしか極値のない関数fにおける極値を探索するアルゴリズム
fn main() {
    input!{p: f64}
    // 問題を解き終える時刻
    let moor = |t: f64| -> f64 {
        t + p / 2.0_f64.powf(t / 1.5)
    };

    // 三分探索で
    let mut l: f64 = 0.0;
    let mut r: f64 = 1e9;
    for _ in 0..100 {
        // lに近い
        let third1: f64 = (l * 2.0 + r) / 3.0;
        // rに近い
        let third2: f64 = (l + r * 2.0) / 3.0;
        // 高いほうの値を下げていく
        if moor(third1) > moor(third2) {
            // third1はrよりもlに近い
            l = third1;
        } else {
            // third2はlよりもrに近い
            r = third2;
        }
    }

    println!("{}", moor(l));
}