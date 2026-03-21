use proconio::input;
use std::collections::{BTreeSet, BTreeMap};
const INF: i64 = 1 << 60;
// ABC430D
// Q. (1) Person at x=xi arrives when index=i (2) What is the sum of the distances from the nearest person?
// A. (1) Segmentation (2) Lower and Upperbound
// = The influence is limited within the segment 「一定の区間のみを考えればいい」 
fn main() {
    input!{n: usize, x: [i64; n]}
    // x座標
    let mut coord = BTreeSet::new();
    coord.insert(0);
    // Point X => Minimum Distance (座標xにある点から最近近傍の点までの距離)
    let mut min_dists = BTreeMap::new();
    min_dists.insert(0, INF);
    let mut cur_sum: i64 = INF;
    let mut answer: Vec<i64> = Vec::new();
    for i in 0..n {
        let mut min_dist: i64 = INF;
        if let Some(&x_lower_bound) = coord.range(..=x[i]).next_back() {
            let min_dist_lower = min_dists[&x_lower_bound];
            let min_dist1: i64 = x[i] - x_lower_bound;
            min_dist = min_dist.min(min_dist1);
            if min_dist1 < min_dist_lower {
                cur_sum += min_dist1 - min_dist_lower;
                min_dists.insert(x_lower_bound, min_dist1);
            }
        }
        if let Some(&x_upper_bound) = coord.range(x[i]..).next() {
            let min_dist_upper = min_dists[&x_upper_bound];
            let min_dist2: i64 = x_upper_bound - x[i];
            min_dist = min_dist.min(min_dist2);
            if min_dist2 < min_dist_upper {
                cur_sum += min_dist2 - min_dist_upper;
                min_dists.insert(x_upper_bound, min_dist2);
            }
        }
        min_dists.insert(x[i], min_dist);
        coord.insert(x[i]);
        cur_sum += min_dist;
        answer.push(cur_sum);
    }

    for ans in answer {
        println!("{}", ans);
    }
}