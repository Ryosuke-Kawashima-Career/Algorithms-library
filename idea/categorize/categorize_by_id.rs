use proconio::input;
// abc359c
// Q. 1x2の大きさの長方形のタイル間を行き来する最小のコストは?
// A. x=0から距離をもとにセルにIDを割り振る
// IDからセル同士の距離を計算する
fn main() {
    input!{sx: i64, sy: i64, tx: i64, ty: i64}
    let height: i64 = (ty - sy).abs();
    let id_s: i64 = id(sx, sy);
    let id_t: i64 = id(tx, ty);
    let id_left = if sy % 2 == 0 {
        id_s - (ty - sy).abs() / 2
    } else {
        id_s - ((ty - sy).abs() + 1) / 2
    };
    let id_right = if sy % 2 == 0 {
        id_s + ((ty - sy).abs() + 1) / 2
    } else {
        id_s + (ty - sy).abs() / 2
    };
    let width = 0.max(id_left - id_t).max(id_t - id_right);
    let ans = height + width;
    println!("{}", ans);
}

fn id(x: i64, y: i64) -> i64 {
    let res = if y % 2 == 0 {
        x / 2
    } else {
        (x + 1) / 2
    };
    return res;
}