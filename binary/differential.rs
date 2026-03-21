use proconio::input;
// arc054b
// 探索の対象の関数が凸関数: 三分探索
// 微分して二分法もしくは, ニュートン法(x1 = x0 - f(x0)/f'(x0))
// 最小値を出すのでf'(t) = 0となるtを求める
fn main() {
    input!{p: f64}
    // 微分する
    let f = |t: f64| -> f64 {
        t + p / 2.0_f64.powf(t / 1.5)
    };
    let df_dt = |t: f64| -> f64 {
        1.0 - 2.0_f64.ln() * p / 1.5 / 2.0_f64.powf(t / 1.5)
    };
    
    // df_dt(t)=0となるtを求める
    let mut l: f64 = 0.0;
    let mut r: f64 = 1e9;
    for _ in 0..100 {
        let mid: f64 = (l + r) / 2.0;
        if df_dt(mid) >= 0.0 {
            r = mid;
        } else {
            l = mid;
        }
    }

    println!("{}", f(r));
}