use proconio::input;
// abc389d
// Q. Consider drawing a circle of radius R centered at the center of one of these squares. How many of these squares are completely contained inside the circle?
// find the number of integer pairs (i,j) such that all four points (i+0.5,j+0.5), (i+0.5,j-0.5), (i-0.5,j+0.5), and (i-0.5,j-0.5) are at a distance of at most R from the origin
// A. x^2 + y^2 <= R^2 -> (2x)^2 + (2*y)^2 <= 4R^2
// Convert float to int!!!
fn main() {
    input!{r: i64}
    // the points whose x or y position is 0. The 1 means the center.
    let mut ans: i64 = 4 * (r - 1) + 1;
    for x in 1..=r {
        let y_square = 4 * r * r - (2 * x + 1) * (2 * x + 1);
        let y = ((y_square as f64).sqrt() as i64 - 1) / 2;
        if y > 0 {
            ans += 4 * y;
        }
    }
    println!("{}", ans);
}