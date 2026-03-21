use proconio::input;
const INF: i64 = 1 << 60;
// 鉄則B24
// LIS: longest Increasing Seq
// 縦x, 横yの箱の重箱みたいな問題
// x軸に対してsort -> y軸に対してLIS
fn main() {
    input!{n: usize, mut xy: [(i64, i64); n]}
    // xi = xj の時 yi >= yj
    xy.sort_by(|a, b| if a.0 == b.0 {
        b.1.cmp(&a.1)
    } else {
        a.0.cmp(&b.0)
    });
    let mut min_top: Vec<i64> = vec![INF; n+1];
    min_top[0] = 0;
    let mut lengths: Vec<usize> = vec![0; n];
    for i in 0..n {
        // lower bound
        let length: usize = min_top.partition_point(|&y| y < xy[i].1);
        lengths[i] = length;
        if length <= n {
            min_top[length] = xy[i].1;
        }
    }
    let ans: usize = *lengths.iter().max().unwrap();
    println!("{}", ans);
}