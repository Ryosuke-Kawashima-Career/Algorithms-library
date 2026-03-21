use proconio::input;
const INF: usize = 1 << 60;
// abc330c
// √全探索: x
// |x^2 + y^2 - D| = a: min a?
fn main() {
    input!{d: usize}
    let d_sqrt: usize = (d as f64).sqrt() as usize;
    let mut ans: usize = INF;

    for x in 0..=(d_sqrt+1) {
        if x * x < d {
            let remain: usize = d - x * x;
            let y: usize = (remain as f64).sqrt() as usize;
            ans = ans.min(remain.abs_diff(y * y));
            ans = ans.min(remain.abs_diff((y+1) * (y+1)));
        } else {
            ans = ans.min(d.abs_diff(x * x));
        }
    }

    println!("{}", ans);
}