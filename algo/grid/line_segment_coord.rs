use proconio::input;
use std::collections::{HashSet, HashMap, BTreeSet};
// abc385d
// Q. how many points of xy do you pass?
// A. manage the 2d coord by HashMap<position x1, BTreeSet<position x2>>
fn main() {
    input!{n: usize, m: usize, sx: i64, sy: i64, xy: [(i64, i64); n], dc: [(char, i64); m]}
    // HashMap<x, BTreeSet<y>>
    let mut coord_x = HashMap::new();
    let mut coord_y = HashMap::new();
    let mut coord_seen = HashSet::new();
    let mut coord = HashSet::new();
    for &(x, y) in xy.iter() {
        (*coord_x.entry(x).or_insert(BTreeSet::new())).insert(y);
        (*coord_y.entry(y).or_insert(BTreeSet::new())).insert(x);
        coord.insert((x, y));
    }
    let mut cur_x = sx;
    let mut cur_y = sy;
    let mut ans: usize = 0;
    if coord.contains(&(cur_x, cur_y)) {
        ans += 1;
        coord_seen.insert((cur_x, cur_y));
    }
    for &(direction, c) in dc.iter() {
        let (next_x, next_y) = if direction == 'U' {
            (cur_x, cur_y + c)
        } else if direction == 'D' {
            (cur_x, cur_y - c)
        } else if direction == 'L' {
            (cur_x - c, cur_y)
        } else {
            (cur_x + c, cur_y)
        };

        // BTreeSetの機能で線分上に点があるかを判定する!!!
        if direction == 'U' || direction == 'D' {
            // move in a line parallel to the X-Axis
            if coord_x.contains_key(&cur_x) {
                for &passed_y in coord_x[&cur_x].range((cur_y.min(next_y))..=(cur_y.max(next_y))) {
                    if !coord_seen.contains(&(cur_x, passed_y)) {
                        ans += 1;
                        coord_seen.insert((cur_x, passed_y));
                    }
                }
            }
        } else {
            // move in a line parallel to the Y-Axis
            if coord_y.contains_key(&cur_y) {
                for &passed_x in coord_y[&cur_y].range((cur_x.min(next_x))..=(cur_x.max(next_x))) {
                    if !coord_seen.contains(&(passed_x, cur_y)) {
                        ans += 1;
                        coord_seen.insert((passed_x, cur_y));
                    }
                }
            }
        }
        cur_x = next_x;
        cur_y = next_y;
    }
    println!("{} {} {}", cur_x, cur_y, ans);
}