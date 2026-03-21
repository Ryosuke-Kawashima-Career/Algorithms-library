use proconio::input;
// abc359c
// Q. 1x2の大きさの長方形のタイル間を行き来する最小のコストは?
// A. 複数の下限を見積もる: 答えはA+B
fn main() {
    input!{mut sx: i64, sy: i64, mut tx: i64, ty: i64}
    // 1. タイルの左端にそろえる
    sx -= (sx + sy) % 2;
    tx -= (tx + ty)  % 2;
    // 2. 変数を置いて方程式を解く
    // a * (+2, 0) + b * (+1, +1) = (dx, dy)
    let lower1: i64 = ((tx - sx).abs() + (ty - sy).abs()) / 2;
    let lower2: i64 = (ty - sy).abs();
    // 3. 下限値同士を比較する
    let ans = lower1.max(lower2);
    println!("{}", ans);
}