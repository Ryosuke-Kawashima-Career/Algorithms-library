use proconio::input;
// square869120Contest #6 B
// 上限を見積もる(入り口と出口の移動時間の最適解 <= 客が使う店から二つ選ぶ時の移動時間)
fn main() {
    input!{n: usize, ab: [(i64, i64); n]}
    let mut entrances: Vec<i64> = Vec::new();
    for i in 0..n {
        entrances.push(ab[i].0);
        entrances.push(ab[i].1);
    }
    entrances.sort();
    let mut upper_bound: i64 = 1 << 60;

    for start_i in 0..entrances.len() {
        for goal_i in start_i..entrances.len() {
            let start: i64 = entrances[start_i];
            let goal: i64 = entrances[goal_i];
            let mut time_sum: i64 = 0;
            for i in 0..n {
                time_sum += (ab[i].0 - start).abs();
                time_sum += (ab[i].1 - ab[i].0).abs();
                time_sum += (goal - ab[i].1).abs();
            }
            upper_bound = upper_bound.min(time_sum);
        }
    }

    println!("{}", upper_bound);
}
// 別解
// ある基準点からの差分の絶対値の和 = 配列の中央値を基準点にした時の絶対距離の和
fn min_abs_is_median() {
    input!{n: usize, ab: [(i64, i64); n]}
    let mut a: Vec<i64> = Vec::new();
    let mut b: Vec<i64> = Vec::new();
    for i in 0..n {
        a.push(ab[i].0);
        b.push(ab[i].1);
    }
    a.sort();
    b.sort();

    let start: i64 = a[n / 2];
    let goal: i64 = b[n / 2];
    let mut time_sum: i64 = 0;
    for i in 0..n {
        // 絶対値を最小にするので中央値を基準にする
        time_sum += (ab[i].0 - start).abs();
        time_sum += (ab[i].1 - ab[i].0).abs();
        time_sum += (goal - ab[i].1).abs();
    }

    println!("{}", time_sum);
}