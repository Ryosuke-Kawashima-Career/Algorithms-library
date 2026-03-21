use proconio::input;
// abc449 D
// Q. lattice Point = 格子点: count the number of lattice points (x, y) such that l <= x <= r and d <= y <= u and max(|x|, |y|) is even
// A. Fix the x-axis and scan the number of y-values.
fn main() {
    input! { l: i64, r: i64, d: i64, u: i64 }
    let mut ans: i64 = 0;

    for x in l..=r {
        let x_abs = x.abs();

        // Region 1: y < -x_abs (where |y| > |x|, so max(|x|, |y|) = |y|)
        // y is between d and min(u, -x_abs - 1)
        let r1_min = d;
        let r1_max = u.min(-x_abs - 1);
        if r1_min <= r1_max {
            ans += count_evens(r1_min, r1_max);
        }

        // Region 2: -x_abs <= y <= x_abs (where |y| <= |x|, so max(|x|, |y|) = |x|)
        let r2_min = d.max(-x_abs);
        let r2_max = u.min(x_abs);
        if r2_min <= r2_max && x_abs % 2 == 0 {
            ans += r2_max - r2_min + 1;
        }

        // Region 3: y > x_abs (where |y| > |x|, so max(|x|, |y|) = |y|)
        // y is between max(d, x_abs + 1) and u
        let r3_min = d.max(x_abs + 1);
        let r3_max = u;
        if r3_min <= r3_max {
            ans += count_evens(r3_min, r3_max);
        }
    }
    println!("{}", ans);
}

// Robustly counts the exact number of even integers in the inclusive range [start, end]
// div_euclid handles negative integers mathematically perfectly, unlike standard `/ 2`
// e.g. -10/3 = 3 where are (-10).div_euclid(3) = -4
fn count_evens(start: i64, end: i64) -> i64 {
    end.div_euclid(2) - (start - 1).div_euclid(2)
}
