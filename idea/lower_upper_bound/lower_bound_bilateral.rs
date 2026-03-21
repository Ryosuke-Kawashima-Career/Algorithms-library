use proconio::input;
use proconio::marker::Chars;
// 鉄則B38
// 下限値を見積もる．
// 双方向から計算することで，全体の下限が分かる．
fn main() {
    input!{n: usize, s: Chars}
    let mut lower_bound_left: Vec<usize> = vec![1; n];
    let mut lower_bound_right: Vec<usize> = vec![1; n];

    // 何回連続で値が増加するか？
    let mut streak_left: usize = 0;
    for i in 1..n {
        // 右方向に単調増加するとき
        if s[i-1] == 'A' {
            streak_left += 1;
            lower_bound_left[i] += streak_left;
        } else {
            streak_left = 0;
        }
    }

    let mut streak_right: usize = 0;
    for i in (0..n-1).rev() {
        // 左方向に単調増加するとき
        if s[i] == 'B' {
            streak_right += 1;
            lower_bound_right[i] += streak_right;
        } else {
            streak_right = 0;
        }
    }

    let mut min_sum: usize = 0;
    for i in 0..n {
        // 左右の下限値のmaxが全体の下限値
        min_sum += lower_bound_left[i].max(lower_bound_right[i]);
    }
    println!("{}", min_sum);
}