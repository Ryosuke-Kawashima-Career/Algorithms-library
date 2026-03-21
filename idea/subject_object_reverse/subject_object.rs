use proconio::input;
// 鉄則90A37
// 主客転倒テクニック(言い換え)
fn main() {
    input!{n: usize, m: usize, b: usize, a: [usize; n], c: [usize; m]}
    // 問題を複数のパーツに分解し、各パーツの寄与分を計算する。
    let sum_a: usize = a.iter().sum::<usize>();
    let sum_c: usize = c.iter().sum::<usize>();
    let sum_b: usize = b*n*m;
    let ans: usize = m * sum_a + sum_b + n * sum_c;
    println!("{}", ans);
}