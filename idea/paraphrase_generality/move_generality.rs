use proconio::input;
// abc359c
// Q. 1x2の大きさの長方形のタイル間を行き来する最小のコストは?
// A. 一般性を失わないように問題を変形する
// 一般性を失わない変形1: [sx,sy]と[tx,ty]をそれぞれ(−sx,−sy)だけ平行移動する
// [0,0]から[tx−sx,ty−sy]への移動を考えればよい
// 一般性を失わない変形2: X座標, Y座標を正に変換する
fn main() {
    input!{mut sx: i64, sy: i64, mut tx: i64, mut ty: i64}
    // タイルの左に寄せる
    sx -= (sy + sx) % 2;
    tx -= (ty + tx) % 2;
    // 平行移動する
    tx -= sx;
    ty -= sy;
    // 座標の第一象限に移動する
    tx = tx.abs();
    ty = ty.abs();

    let ans = ty + 0.max(tx - ty) / 2;
    println!("{}", ans);
}