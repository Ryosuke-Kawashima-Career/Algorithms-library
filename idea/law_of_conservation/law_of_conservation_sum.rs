use proconio::input;
// abc313C
// 数列の和の保存則
// 1つの数を+1, もう1つを-1することで|max - min| <= 1
fn main() {
    input!{n: usize, mut a: [i64; n]}
    // 昇順に並べ，下位の数値を最小値に，上位の数値を最大値にする!
    a.sort();
    let sum_a: i64 = a.iter().sum();
    let (min_a, max_a) = if sum_a % n as i64 == 0 {
        (sum_a / n as i64, sum_a / n as i64)
    } else {
        (sum_a / n as i64, sum_a / n as i64 + 1)
    };
    let num_of_max = sum_a as usize - min_a as usize * n;

    let mut operations = 0;
    for i in 0..n {
        if i < n  - num_of_max {
            operations += (a[i] - min_a).abs();
        } else {
            operations += (max_a - a[i]).abs();
        }
    }
    // +1と-1で数字が一回の操作で2動くので2で割る
    println!("{}", operations / 2);
}