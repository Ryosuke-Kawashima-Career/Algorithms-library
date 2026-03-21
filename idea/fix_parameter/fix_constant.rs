use proconio::input;
// abc432c
// Q. xi candies of weight(y) => Solve the following Equation.
// Q. X*(a[i]-x[i]) + Y*x[i] == X*(a[0] - x0) + Y*x0
// A. ある変数を決め打って考えてみる: 定数に注目してみる。
// A. 1. まずはソートして計算しやすいようにする。2. 変数の決め打ちを行う。3. 変数を決めうった後に、連立方程式を解くことで、求めたい変数を表現する。4. その変数の表現に必要な制約条件をmodや境界条件に着目して列挙する。
fn main() {
    input!{n: usize, x: i64, y: i64, mut a: [i64; n]}
    a.sort();
    let ans = solve(x, y, &a);
    println!("{}", ans);
}

fn solve(x: i64, y: i64, a: &Vec<i64>) -> i64 {
    /* a[] is already sorted ascendingly. */
    let y_x: i64 = y - x;
    let n: usize = a.len();
    let mut heavies: Vec<i64> = vec![0; n];
    heavies[0] = a[0];
    for i in 1..n {
        if y * (a[i] - a[0]) % y_x != 0 {
            return -1;
        }
        let diff: i64 = y * (a[i] - a[0]) / y_x;
        heavies[i] = a[i] - diff;
        if heavies[i] < 0 {
            return -1;
        }
    }
    let sum_y: i64 = heavies.iter().sum();
    return sum_y;
}