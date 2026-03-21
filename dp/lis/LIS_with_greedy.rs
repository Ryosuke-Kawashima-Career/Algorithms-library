use proconio::input;
// ABC439E
// Q. Person i is standing at point (Ai,0) and trying to fly a kite at point (Bi,1).
// Q. Count the maximum number of segments that can be flown without intersecting.
// A1-1. Greedy Algorithm = The smaller A the better!!!
// (Ai,Bi) の列を Ai 昇順 に並べ替える。ただし Ai の値が一致する時は Bi 降順 で並べ替える。
// A1-2. LIS(Longest Increasing Subsequence) = DP
fn main() {
    input! {
        n: usize,
        mut ab: [(i64, i64); n],
    }

    // Sort by A ascending; for equal A, sort by B descending.
    // This allows us to reduce the problem to LIS on B.
    // Sorting B descending for equal A ensures we don't pick multiple
    // segments with the same start point (which would intersect).
    ab.sort_unstable_by(|a, b| {
        if a.0 != b.0 {
            // ascending
            a.0.cmp(&b.0)
        } else {
            // descending
            b.1.cmp(&a.1)
        }
    });

    // LIS(Longest Increasing Subsequence) = DP
    let mut tails = Vec::new();
    for (_, b) in ab {
        // Find the first element in tails that is >= b by Lower Bound
        let idx = tails.partition_point(|&x| x < b);
        if idx == tails.len() {
            tails.push(b);
        } else {
            tails[idx] = b;
        }
    }

    println!("{}", tails.len());
}
