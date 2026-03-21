use proconio::input;
// 鉄則B42
// 絶対値の展開 |a + b| = a + b or -a -b
// 絶対値の展開の場合分け: |表の総和| + |裏の総和| -> 2 * 2 = 4通り
fn main() {
    input!{n: usize, ab: [(i64, i64); n]}

    // (+, +), (+, -), (-, +), (-, -)
    let mut sum_pp: i64 = 0;
    let mut sum_pn: i64 = 0;
    let mut sum_np: i64 = 0;
    let mut sum_nn: i64 = 0;
    for i in 0..n {
        if ab[i].0 + ab[i].1 >= 0 {
            sum_pp += ab[i].0 + ab[i].1;
        }
        if ab[i].0 - ab[i].1 >= 0 {
            sum_pn += ab[i].0 - ab[i].1;
        }
        if -ab[i].0 + ab[i].1 >= 0 {
            sum_np += -ab[i].0 + ab[i].1;
        }
        if -ab[i].0 - ab[i].1 >= 0 {
            sum_nn += -ab[i].0 - ab[i].1;
        }
    }

    let mut max_sum: i64 = 0;
    max_sum = max_sum.max(sum_pp);
    max_sum = max_sum.max(sum_pn);
    max_sum = max_sum.max(sum_np);
    max_sum = max_sum.max(sum_nn);
    println!("{}", max_sum);
}