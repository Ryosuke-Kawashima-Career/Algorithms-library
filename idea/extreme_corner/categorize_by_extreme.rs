use proconio::input;
const RAD_TO_DEG: f64 = std::f64::consts::FRAC_1_PI * 180.0;
// abc144d
// 極端な場合を考えて場合分けする
// 水が全くないときと、ほぼ満タンなとき
// 水槽に水を入れたときに水がこぼれない傾きを求める
fn main() {
    // V = a * a * b(height), x = water
    input!{a: f64, b: f64, x: f64}
    let area: f64 = x / a;
    let half: f64 = a * b / 2.0;
    // if water is too little
    let tilt = if area <= half {
        let c = 2.0 * area / b;
        let tan = b / c;
        tan.atan().abs()
    } else {
        // if water is too much
        let remain_area = a * b - area;
        let tan = 2.0 * remain_area / (a * a);
        tan.atan().abs()
    };

    println!("{}", tilt * RAD_TO_DEG);
}