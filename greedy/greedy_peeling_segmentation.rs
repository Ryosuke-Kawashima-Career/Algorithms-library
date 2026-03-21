use proconio::input;
use std::collections::HashMap;
// ABC445D
// Q. Break down the one chocolate by hand without rotation
// A. Greedy Peeling
/* Key Point:
Repeatedly placing a piece whose height or width coincides with those of the remaining empty region.
It suffices to repeatedly fetch a piece with the maximum height and a piece with the maximum width, and place whatever fits.
= 左上から順に詰めていけばいい。
 */
fn main() {
    input! {h: usize, w: usize, n: usize, hw: [(usize, usize); n]}

    // Store pieces by height and width for O(1) access
    let mut by_h: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut by_w: HashMap<usize, Vec<usize>> = HashMap::new();

    for (i, &(r, c)) in hw.iter().enumerate() {
        by_h.entry(r).or_insert(Vec::new()).push(i);
        by_w.entry(c).or_insert(Vec::new()).push(i);
    }

    let mut current_h = h;
    let mut current_w = w;
    let mut row_offset = 0;
    let mut col_offset = 0;

    let mut ans = vec![(0, 0); n];
    let mut used = vec![false; n]; // Track used pieces

    for _ in 0..n {
        let mut placed = false;

        // Strategy 1: Find a piece matching current Height (Vertical strip)
        while let Some(idx) = by_h.get_mut(&current_h).and_then(|v| v.pop()) {
            if used[idx] {
                continue;
            }

            let piece_w = hw[idx].1;
            ans[idx] = (row_offset, col_offset);
            used[idx] = true;

            col_offset += piece_w;
            current_w -= piece_w;
            placed = true;
            break;
        }

        if placed {
            continue;
        }

        // Strategy 2: Find a piece matching current Width (Horizontal strip)
        while let Some(idx) = by_w.get_mut(&current_w).and_then(|v| v.pop()) {
            if used[idx] {
                continue;
            }

            let piece_h = hw[idx].0;
            ans[idx] = (row_offset, col_offset);
            used[idx] = true;

            row_offset += piece_h;
            current_h -= piece_h;
            placed = true;
            break;
        }
    }

    for (r, c) in ans {
        println!("{} {}", r + 1, c + 1);
    }
}
