use proconio::input;
use std::collections::BTreeSet;
// arc092C: 2D Plane 2N Points
// 平面走査: 2次元の座標点を扱うテクニック
// 座標点をある一方の座標、例えば x座標でソートして小さい順番に見ていき、
// もう一方の y座標についてはデータ構造などを用いて処理していくテクニック
// scanline algorithm: 軸に平行な直線をスライドさせて平面全体を走査していく
// Q. 赤い点と青い点は，赤い点のx座標が青い点のx座標より小さく
// また赤い点のy座標も青い点のy座標より小さいとき仲良しペア
fn main() {
    input!{n: usize, coord_red: [(i64, i64); n], coord_blue: [(i64, i64); n]}
    let mut coord: Vec<(i64, i64, usize)> = Vec::new();
    for i in 0..n {
        coord.push((coord_red[i].0, coord_red[i].1, 0));
        coord.push((coord_blue[i].0, coord_blue[i].1, 1));
    }
    // x軸でソート
    coord.sort();
    let mut y_red = BTreeSet::new();

    // X軸から走査し，yが小さい値をなるべく選ぶ
    let mut pairs: usize = 0;
    for &(_, y, color) in coord.iter() {
        // 0: 赤, 1: 青
        if color == 0 {
            y_red.insert(y);
        } else {
            if let Some(&y_max) = y_red.range(..y).next_back() {
                pairs += 1;
                y_red.remove(&y_max);
            }
        }
    }

    println!("{}", pairs);
}