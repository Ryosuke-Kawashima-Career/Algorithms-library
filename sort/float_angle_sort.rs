fn main() {
    /* Note that this is a WA Answer of ABC442E */
    input! {n: usize, q: usize, xy: [(i64, i64); n], ab: [(Usize1, Usize1); q]}
    // 1. Precalculate angles for all points in original order to answer queries
    let mut angles: Vec<f64> = Vec::with_capacity(n);
    for i in 0..n {
        angles.push((xy[i].1 as f64).atan2(xy[i].0 as f64));
    }

    // 2. Create a sorted version for binary search
    let mut sorted_angles = angles.clone();
    sorted_angles.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // 3. Handle cyclic nature: Append the same angles + 2*PI
    // This allows us to handle ranges that wrap around (e.g. 170 to -170 degrees) easily
    let pi2 = 2.0 * std::f64::consts::PI;
    for i in 0..n {
        sorted_angles.push(sorted_angles[i] + pi2);
    }

    let mut answers = Vec::new();
    for query in 0..q {
        let idx1 = ab[query].0;
        let idx2 = ab[query].1;

        let theta_start = angles[idx1];
        let mut theta_end = angles[idx2];

        // Ensure we measure counter-clockwise from start to end
        // If end < start, it means we wrapped around the cut, so add 2*PI to end
        if theta_end < theta_start {
            theta_end += pi2;
        }

        // 4. Use binary search (partition_point)
        // Find first element >= theta_start
        let lower = sorted_angles.partition_point(|&x| x < theta_start);
        // Find first element > theta_end
        let upper = sorted_angles.partition_point(|&x| x <= theta_end);

        // The count is the difference
        let count = upper - lower;
        answers.push(count);
    }

    for ans in answers {
        println!("{}", ans);
    }
}
