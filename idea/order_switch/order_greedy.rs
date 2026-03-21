use proconio::input;
// abc127D
// A[B個選ぶ] -> C　の操作で合計値を最大化する!
// hint. 操作の順番が定められているが、どの順番で操作を行っても答えは変わらない
// A. 配列のうち小さい要素からBCの大きな要素で書き換えていく
fn main() {
    input!{n: usize, m: usize, mut a: [i64; n], mut bc: [(usize, i64); m]}
    // 貪欲法
    a.sort();
    bc.sort_by(|x, y| y.1.cmp(&x.1));
    let mut cur_index: usize = 0;
    for &(b, c) in bc.iter() {
        if cur_index >= n {
            break;
        }
        for i in cur_index..cur_index+b {
            if i < n && a[i] < c {
                a[i] = c;
            }
        }
        cur_index += b;
    }

    let ans = a.iter().sum::<i64>();
    println!("{}", ans);
}