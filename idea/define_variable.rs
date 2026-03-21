use proconio::input;
// abc359c
// Q. 1x2の大きさの長方形のタイル間を行き来する最小のコストは?
// A. 下限を見積もる: 答えはA+B
// 上下方向に1移動し、左右方向に1移動する: A回
// 左右方向に2移動する: B回　と文字で置くと
// |Sy - Ty| <= A かつ |Sx - Tx| <= A + 2*B
fn main() {
    input!{mut sx: i64, sy: i64, mut tx: i64, ty: i64}
    // タイルの左に寄る
    move_left(&mut sx, sy);
    move_left(&mut tx, ty);

    // 答えは (|Sy - Ty| + max(|Sx - Tx|, |Sy - Ty|)) / 2
    let dx = (sx - tx).abs();
    let dy = (sy - ty).abs();
    let ans = (dy + dy.max(dx)) / 2;
    println!("{}", ans);
}

fn move_left(x: &mut i64, y: i64) {
    if (*x + y) % 2 == 1 {
        *x -= 1;
    }
}