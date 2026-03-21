use proconio::input;
use proconio::marker::Chars;
use std::collections::BTreeSet;
// abc218C(abc322Dの類題)
// グリッド上のものを(x, y)のsetで計算する
// 平行移動は、各要素から値を足し引きするだけ
fn main() {
    input!{n: usize, s: [Chars; n], t: [Chars; n]}
    // 座標の集合で形状を保存する
    let mut coord_s: Vec<(i64, i64)> = Vec::new();
    // setを使うことで座標に点が存在するか高速に計算できる
    let mut coord_t = BTreeSet::new();
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '#' {
                coord_s.push((i as i64, j as i64));
            }
            if t[i][j] == '#' {
                coord_t.insert((i as i64, j as i64));
            }
        }
    }

    let mut is_same: bool = false;
    for _rot in 0..4 {
        if is_matched(&coord_s, &coord_t) {
            is_same = true;
        }
        rotate(&mut coord_s);
    }

    if is_same {
        println!("Yes");
    } else {
        println!("No");
    }
}
// (x, y) -> clock: (y, -x), anti: (-y, x)
fn rotate(grid: &mut Vec<(i64, i64)>) {
    let n: usize = grid.len();
    for i in 0..n {
        let (y, x) = grid[i];
        grid[i] = (x, -y);
    }
}

// 最小値同士を比較して平行移動する
fn is_matched(coord: &Vec<(i64, i64)>, target: &BTreeSet<(i64, i64)>) -> bool {
    let coord_set: BTreeSet<_> = coord.iter().collect();
    let (min_y, min_x) = coord_set.iter().next().unwrap();
    let (target_min_y, target_min_x) = target.iter().next().unwrap();
    let (dif_y, dif_x) = (target_min_y - min_y, target_min_x - min_x);
    let moved_coord_set: BTreeSet<_> = coord_set.iter().map(|&(y, x)| (y+dif_y, x+dif_x)).collect();
    moved_coord_set == *target
}