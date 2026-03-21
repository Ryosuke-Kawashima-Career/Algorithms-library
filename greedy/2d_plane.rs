use proconio::input;
use proconio::derive_readable;

#[derive_readable]
#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize, y: usize
}
// arc092C: 2D Plane 2N Points
// blue > red in x, y
fn main() {
    input!{n: usize, mut reds: [Point; n], mut blues: [Point; n]}
    // xで並び替える。
    reds.sort_by_key(|point| point.x);
    blues.sort_by_key(|point| point.x);
    let mut red_used: Vec<bool> = vec![false; n];
    let mut ans: usize = 0;

    // blue
    for i in 0..n {
        let mut max_index: usize = n;
        let mut max_y: usize = 0;
        // red
        for j in 0..n {
            if reds[j].x >= blues[i].x {
                break;
            }
            
            // x, yの値が最大の赤を探す。ゆえに=<
            if reds[j].y < blues[i].y && !red_used[j] && max_y <= reds[j].y {
                max_index = j;
                max_y = reds[j].y;
            }
        }

        if max_index != n {
            red_used[max_index] = true;
            ans += 1;
        }
    }

    println!("{}", ans);
}