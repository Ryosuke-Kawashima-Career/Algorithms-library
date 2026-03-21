use proconio::input;
// abc449 D
// Q. lattice Point = 格子点: count the number of lattice points (x, y) such that l <= x <= r and d <= y <= u and max(|x|, |y|) is even
// A. Distinguish the cases regarding the big-small relations between |x| and |y|
fn main() {
    input! { l: i64, r: i64, d: i64, u: i64 }
    let mut ans: i64 = 0;
    // case1: |x| > |y| => max(|x|, |y|) = |x| => |x| is even
    for x in l..=r {
        if x.abs() % 2 == 0 {
            let lower_bound = d.max(-x.abs() + 1);
            let upper_bound = u.min(x.abs() - 1);
            let count_x: i64 = upper_bound - lower_bound + 1;
            ans += count_x.max(0);
        }
    }
    // case2: |x| <= |y| => max(|x|, |y|) = |y| => |y| is even
    for y in d..=u {
        if y.abs() % 2 == 0 {
            // check there are no duplicates for the counts
            let lower_bound = l.max(-y.abs());
            let upper_bound = r.min(y.abs());
            let count_y: i64 = upper_bound - lower_bound + 1;
            ans += count_y.max(0);
        }
    }
    println!("{}", ans);
}
