use proconio::input;
const MAXTIME: usize = 200_000;
// 鉄則C15
// 累積と貪欲法を左右から行う。
fn main() {
    input!{n: usize, k: usize, lr: [(usize, usize); n]}
    let mut times: Vec<(usize, usize)> = Vec::new();
    for i in 0..n {
        times.push((lr[i].0, lr[i].1 + k));
    }
    let mut times_original = times.clone();
    times.sort_by(|x, y| x.1.cmp(&y.1));
    // 出席した会議数の累積和を取る
    let mut count_left: Vec<usize> = vec![0; MAXTIME];
    let mut count_right: Vec<usize> = vec![0; MAXTIME];

    // まず、普通の貪欲法
    let mut cur_time_left: usize = 0;
    let mut cur_cnt_left: usize = 0;
    for i in 0..n {
        if cur_time_left <= times[i].0 {
            // 会議が終わって初めて出席したことになる．
            count_left[times[i].1] = cur_cnt_left + 1;
            cur_time_left = times[i].1;
            cur_cnt_left += 1;
        }
    }

    // 逆から計算する(時をさかのぼる)
    // 右からの実装は左からの実装をを反転する
    times.sort_by(|x, y| y.0.cmp(&x.0));
    let mut cur_time_right: usize = MAXTIME;
    let mut cur_cnt_right: usize = 0;
    for i in 0..n {
        if cur_time_right >= times[i].1 {
            count_right[times[i].0] = cur_cnt_right + 1;
            cur_time_right = times[i].0;
            cur_cnt_right += 1;
        }
    }

    // 累積を取る(最大何回出席できるかを計算: max)
    for time in 0..MAXTIME {
        if time > 0 {
            count_left[time] = count_left[time].max(count_left[time - 1]);
        }
    }
    for time in (0..MAXTIME).rev() {
        if time + 1 < MAXTIME {
            count_right[time] = count_right[time].max(count_right[time + 1]);
        }
    }

    for i in 0..n {
        // 会議の前 + 現在の会議 + 会議の後
        let meetings: usize = count_left[times_original[i].0] + 1 + count_right[times_original[i].1];
        println!("{}", meetings);
    }
}