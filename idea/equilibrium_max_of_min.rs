use proconio::input;
// Square869120Contest #5 B - Emblem
// 最小の半径の最大化
fn main() {
    input!{n: usize, m: usize, xyr: [(f64, f64, f64); n], xy: [(f64, f64); m]}
    let mut min_radius: f64 = f64::MAX;
    for &(_, _, r) in xyr.iter() {
        min_radius = min_radius.min(r);
    }
    // xyrとxyの組み合わせ
    for i in 0..n {
        for j in 0..m {
            let x_sq: f64 = (xyr[i].0 - xy[j].0) * (xyr[i].0 - xy[j].0);
            let y_sq: f64 = (xyr[i].1 - xy[j].1) * (xyr[i].1 - xy[j].1);
            let dist: f64 = (x_sq + y_sq).sqrt();
            let radius: f64 = dist - xyr[i].2;
            min_radius = min_radius.min(radius);
        }
    }
    // xyとxyの組み合わせ
    // xyの半径は二つの中心の距離の半分になればよい!!!
    // 一方が大きくなると他方が小さくなる -> 中点で半径の値が均衡する
    for i in 0..m {
        for j in i+1..m {
            let x_sq: f64 = (xy[i].0 - xy[j].0) * (xy[i].0 - xy[j].0);
            let y_sq: f64 = (xy[i].1 - xy[j].1) * (xy[i].1 - xy[j].1);
            let dist: f64 = (x_sq + y_sq).sqrt();
            let radius: f64 = dist / 2.0;
            min_radius = min_radius.min(radius);
        }
    }
    println!("{}", min_radius);
}