use proconio::input;
// 典型70 (square869120Contest #6でも使える)
// xとyが独立しているのでそれぞれの軸に関して計算する。
// 中央値が周りからの差分の合計が最小値になる。
fn main() {
    input!{n: usize}
    let mut x_axis: Vec<i64> = Vec::new();
    let mut y_axis: Vec<i64> = Vec::new();
    
    for _ in 0..n {
        input!{x: i64, y: i64}
        x_axis.push(x);
        y_axis.push(y);
    }
    x_axis.sort();
    y_axis.sort();
    let x_med: i64 = median(&x_axis);
    let y_med: i64 = median(&y_axis);
    let mut ans: i64 = 0;

    for i in 0..n {
        ans += (x_axis[i]-x_med).abs();
        ans += (y_axis[i]-y_med).abs();
    }

    println!("{}", ans);
}

fn median(a: &Vec<i64>) -> i64 {
    let n: usize = a.len();
    let med: usize = n/2;
    return a[med];
}