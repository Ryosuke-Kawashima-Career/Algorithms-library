use proconio::input;
// ABC433A
// Q. Is there a "k" s.t. x + k = z * (y + k)? (Z>=2)
// A. Change the equation to 1: k = (zy - x) / (z - 1)
// 2: x <= x - 2y < k to meet z >= 2
// => Expression Transformation!!! = 式変形
fn main() {
    input! {x: usize, y: usize, z: usize}
    if x >= z * y && (x - z * y) % (z - 1) == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
