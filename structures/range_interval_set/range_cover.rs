use proconio::input;
use std::collections::BTreeSet;
// ABC435E
// Q. lr continues to come, paint [l, r) to black. After each query, output how many white cells remain.
// A. BTreeSet manages the ranges dynamically.
// we manage the white segements
// Black is sandwiched between white segments!!!: [white][black][white]
fn main() {
    input! {n: usize, q: usize, lr: [(i64, i64); q]}
    // initially all white
    let mut white_segments = BTreeSet::from([(0, n as i64)]);
    let mut total_white = n as i64;
    for query in 0..q {
        let (l, r) = lr[query];
        let l = l - 1; // [l, r)
        let (mut min_white_l, mut max_white_r) = (l, r);
        let mut ranges_to_remove = Vec::new();
        // find all white segments that intersect with [l, r).
        for &(wl,  wr) in white_segments.range((..(r, n as i64))).rev() {
            // no intersection
            if wr <= l {
                break;
            }
            total_white -=  wr - wl;
            ranges_to_remove.push((wl, wr));
            // extend black segment
            min_white_l = min_white_l.min(wl);
            max_white_r = max_white_r.max(wr);
        }
        // remove the white segments that intersect with [l, r)
        for range_to_remove in ranges_to_remove {
            white_segments.remove(&range_to_remove);
        }
        // [min_white_l, black_l) と [black_r, max_white_r) を白として追加する
        total_white += (l - min_white_l).max(0) + (max_white_r - r).max(0);
        if min_white_l < l {
            white_segments.insert((min_white_l, l));
        }
        if r < max_white_r {
            white_segments.insert((r, max_white_r));
        }
        println!("{}", total_white);
    }
}