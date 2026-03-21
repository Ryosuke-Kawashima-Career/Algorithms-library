use proconio::{input, marker::Usize1};
// ABC442E
// Use Product to sort by angle on the unit circle.
fn main() {
    input! {n: usize, q: usize, xy: [(i64, i64); n], ab: [(Usize1, Usize1); q]}
    let points = xy.clone();
    let mut p_indices: Vec<usize> = (0..n).collect();

    // Sort based on Clockwise angle starting from (1, 0)
    // To sort Clockwise:
    // Upper half (including positive x-axis) vs Lower half
    //    Actually, for CW starting from (1, 0) (East):
    //    - East to South (Quad 4): y < 0
    //    - South to West (Quad 3): y < 0, x < 0
    //    - West to North (Quad 2): y > 0
    //    - North to East (Quad 1): y > 0, x > 0
    //    So we define region:
    //    if y < 0 {
    //       if x < 0 { 0 } // Q3 (-PI to -PI/2)
    //       else { 1 }     // Q4 (-PI/2 to 0)
    //    } else {
    //       if x >= 0 { 2 } // Q1 (0 to PI/2)
    //       else { 3 }      // Q2 (PI/2 to PI)
    //    }

    p_indices.sort_by(|&i, &j| {
        let (x1, y1) = points[i];
        let (x2, y2) = points[j];

        let q1 = if y1 < 0 {
            if x1 < 0 {
                0
            } else {
                1
            }
        } else {
            if x1 >= 0 {
                2
            } else {
                3
            }
        };
        let q2 = if y2 < 0 {
            if x2 < 0 {
                0
            } else {
                1
            }
        } else {
            if x2 >= 0 {
                2
            } else {
                3
            }
        };

        if q1 != q2 {
            q1.cmp(&q2)
        } else {
            // Same region. Use cross product.
            // (x1*y2 - x2*y1). Diff > 0 => x1*y2 > x2*y1.
            0.cmp(&(x1 * y2 - x2 * y1))
        }
    });

    // Create the sorted points array for querying (duplicated for cyclic)
    let mut sorted_points = Vec::with_capacity(2 * n);
    for &idx in &p_indices {
        sorted_points.push(points[idx]);
    }
    for &idx in &p_indices {
        sorted_points.push(points[idx]);
    }

    let get_quad = |p: (i64, i64)| -> i32 {
        let (x, y) = p;
        if y < 0 {
            if x < 0 {
                0
            } else {
                1
            }
        } else {
            if x >= 0 {
                2
            } else {
                3
            }
        }
    };
    let cross = |a: (i64, i64), b: (i64, i64)| -> i64 { a.0 * b.1 - a.1 * b.0 };
    let cmp_pts = |a: (i64, i64), b: (i64, i64)| {
        let q1 = get_quad(a);
        let q2 = get_quad(b);
        if q1 != q2 {
            q1.cmp(&q2)
        } else {
            0.cmp(&(cross(a, b)))
        }
    };

    let sorted_points_n: Vec<(i64, i64)> = p_indices.iter().map(|&i| points[i]).collect();

    let mut answers = Vec::new();
    for query in 0..q {
        let idx_a = ab[query].0;
        let idx_b = ab[query].1;
        let p_a = points[idx_a];
        let p_b = points[idx_b];

        let lower_b =
            sorted_points_n.partition_point(|&p| cmp_pts(p, p_b) == std::cmp::Ordering::Less);
        let upper_a =
            sorted_points_n.partition_point(|&p| cmp_pts(p, p_a) != std::cmp::Ordering::Greater);

        if cmp_pts(p_a, p_b) == std::cmp::Ordering::Less {
            // Wrapped: [B, End] + [Start, A]
            // Count = (Points >= B) + (Points <= A)
            // Points >= B is (N - lower_b)
            // Points <= A is (upper_a)
            answers.push((n - lower_b) + upper_a);
        } else {
            // Normal: [B, A]
            // Points in [B, A]
            // upper_a is index after A. lower_b is index at B.
            // count = upper_a - lower_b
            if upper_a >= lower_b {
                answers.push(upper_a - lower_b);
            } else {
                // This shouldn't happen if p_a >= p_b, unless logic error.
                answers.push(0);
            }
        }
    }

    for ans in answers {
        println!("{}", ans);
    }
}
