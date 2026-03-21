use proconio::{input, marker::Usize1};
use std::collections::{BTreeSet, BTreeMap};
// abc386d
// Q. すべての行について以下の条件が成り立つ。ある整数 i (0≤i≤N) が存在して、その行の左から i 個のマスは黒、それ以外は白で塗られている。
// すべての列について以下の条件が成り立つ。ある整数 i (0≤i≤N) が存在して、その列の上から i 個のマスは黒、それ以外は白で塗られている。
// つまり，黒が左上，白が右下に位置する
// A. Coord = BTreeMap<x, BTreeSet<y>>
fn main() {
    input!{n: usize, m: usize, xyc: [(Usize1, Usize1, char); m]}
    let mut coord = BTreeMap::new();
    // 黒を先にすべて配置する
    for &(x, y, c) in xyc.iter() {
        if c == 'B' {
            (*coord.entry(x).or_insert(BTreeSet::new())).insert(y);
        }
    }
    for &(x, y, c) in xyc.iter() {
        if c == 'W' {
            // 黒が白の右下に存在する
            if let Some((_, coord_y)) = coord.range(x..).next() {
                if let Some(_) = coord_y.range(y..).next() {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}