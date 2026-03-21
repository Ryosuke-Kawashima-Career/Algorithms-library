use proconio::{input, marker::Usize1};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    fn get_argument(&self) -> f64 {
        (self.y as f64).atan2(self.x as f64)
    }
    fn cross(&self, other: Self) -> i64 {
        self.x * other.y - self.y * other.x
    }
    fn compare(&self, other: Self) -> std::cmp::Ordering {
        /* Compares the points by counterclockwise order */
        // Upper half (y > 0) or Positive X-Axis (y=0, x>0) -> False
        // Lower half (y < 0) or Negative X-Axis (y=0, x<0) -> True
        let quad_self: bool = self.y < 0 || (self.y == 0 && self.x < 0);
        let quad_other: bool = other.y < 0 || (other.y == 0 && other.x < 0);
        if quad_self != quad_other {
            return quad_self.cmp(&quad_other);
        }
        // cross(a, b) > 0 implies a is CCW relative to b (a < b)
        // We want Ordering::Less if cross > 0
        // 0.cmp(&positive) is Less
        0.cmp(&self.cross(other))
    }
}

fn argument_sort(points: &mut [Point]) {
    points.sort_by(|a, b| a.compare(*b));
}
// ABC442 E
// Q. Find the number of points in the range [a, b] in the counterclockwise order
// A. Argument sort + Lower Upper Bound
fn main() {
    input! {n: usize, q: usize, xy: [(i64, i64); n], ab: [(Usize1, Usize1); q]}
    // points[ord_clockwise[i]]
    let points: Vec<Point> = xy.iter().map(|(x, y)| Point::new(*x, *y)).collect();
    // Original indexes -> Current indexes
    let mut ord_clockwise = (0..n).collect::<Vec<usize>>();
    ord_clockwise.sort_by(|a, b| points[*b].compare(points[*a]));
    // Current indexes -> Original indexes
    let mut rev_clockwise = (0..n).collect::<Vec<usize>>();
    for i in 0..n {
        rev_clockwise[ord_clockwise[i]] = i;
    }
    let mut lower_bounds: Vec<usize> = vec![0; n];
    let mut upper_bounds: Vec<usize> = vec![n; n];
    for i in 1..n {
        let cur_point = points[ord_clockwise[i]];
        let prev_point = points[ord_clockwise[i - 1]];
        // cur_point is located counterclockwise to prev_point
        if cur_point.compare(prev_point) == std::cmp::Ordering::Less {
            lower_bounds[i] = i;
        } else {
            lower_bounds[i] = lower_bounds[i - 1];
        }
    }
    for i in (0..n - 1).rev() {
        let cur_point = points[ord_clockwise[i]];
        let next_point = points[ord_clockwise[i + 1]];
        // next_point is located counterclockwise to cur_point
        if next_point.compare(cur_point) == std::cmp::Ordering::Less {
            upper_bounds[i] = i + 1;
        } else {
            upper_bounds[i] = upper_bounds[i + 1];
        }
    }

    for query in 0..q {
        let (idx_a, idx_b) = ab[query];
        // Find group start/end in the Sorted (CW) array
        let start_a = lower_bounds[rev_clockwise[idx_a]];
        let end_b = upper_bounds[rev_clockwise[idx_b]];

        if start_a < end_b {
            println!("{}", end_b - start_a);
        } else {
            println!("{}", n - start_a + end_b);
        }
    }
}
