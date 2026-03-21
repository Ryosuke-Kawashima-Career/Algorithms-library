use proconio::input;
use std::collections::BTreeMap;
// ABC444E
// Q. Find the number of pairs (l, r) such that all the pairs of (l <= i < j <= r) satisfies |a[i] - a[j]| >= d
// A. Sliding Window + BTreeMap = 尺取り法
// Tips: 区間内のすべての要素の組み合わせが所与の条件を満たさなければいけないので、観測の範囲を徐々にずらせばよい。
fn main() {
    input! {n: usize, d: i64, a: [i64; n]}
    let pairs = shakutori_two_pointers(n, d, &a);
    println!("{}", pairs);
}

fn shakutori_two_pointers(n: usize, diff: i64, a: &[i64]) -> usize {
    let mut left = 0;
    let mut ans = 0;
    // Map stores Value -> Count
    let mut window: BTreeMap<i64, usize> = BTreeMap::new();

    for right in 0..n {
        let val = a[right];
        *window.entry(val).or_insert(0) += 1;

        // While the window is invalid, shrink from left
        while !is_valid(val, diff, &window) {
            let left_val = a[left];
            if let Some(count) = window.get_mut(&left_val) {
                *count -= 1;
                if *count == 0 {
                    window.remove(&left_val);
                }
            }
            left += 1;
        }

        // Window [left, right] is valid
        ans += right - left + 1;
    }
    ans
}

fn is_valid(val: i64, diff: i64, window: &BTreeMap<i64, usize>) -> bool {
    // 1. Check duplicates
    if let Some(&count) = window.get(&val) {
        if diff > 0 && count > 1 {
            return false;
        }
    }

    // 2. Check Predecessor
    // range(..val) gets keys strictly less than val
    if let Some((&pred, _)) = window.range(..val).next_back() {
        if val - pred < diff {
            return false;
        }
    }

    // 3. Check Successor
    // range(val+1..) gets keys strictly greater than val
    if let Some((&succ, _)) = window.range(val + 1..).next() {
        if succ - val < diff {
            return false;
        }
    }

    true
}
