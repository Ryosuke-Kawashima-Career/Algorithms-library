use proconio::input;
use std::collections::BTreeMap;
// abc429d
// Q. People are standing on a circular track of length m at various distances.
// A. the problem of spanning => Shakutori Method
// A. Segmentation: 人の位置で区切る
fn main() {
    input!{n: usize, m: i64, c: usize, a: [i64; n]}
    // b: Distances without overlap
    let mut b: Vec<i64> = a.clone();
    b.sort();
    b.dedup();
    let mut distance_count: BTreeMap<i64, usize> = BTreeMap::new();
    for i in 0..n {
        *distance_count.entry(a[i]).or_insert(0) += 1;
    }
    // p: Counts the number of People at the same position
    let mut p: Vec<usize> = Vec::new();
    for (_, &count) in distance_count.iter() {
        p.push(count);
    }
    let k: usize = b.len();
    // Index of the Segmentation
    let mut r: usize = 0;
    let mut current_people: usize = 0;
    let mut ans: usize = 0;
    for l in 0..k {
        while current_people < c {
            // Circulation => Modulo
            current_people += p[r % k];
            r += 1;
            // Circulation => Modulo
            if r > k {
                r %= k;
            }
        }
        // The condition of boundary: 境界条件
        if l == 0 {
            // (Distance: Start - End) × Number of People
            ans += (m + b[0] - b[k - 1]) as usize * current_people;
        } else {
            ans += (b[l] - b[l-1]) as usize * current_people;
        }
        current_people -= p[l];
    }
    println!("{}", ans);
}