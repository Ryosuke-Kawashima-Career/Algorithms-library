use proconio::input;
// ABC415D
// Q. empty A bottles are converted into B new bottles.
// A. Greedy Algorithm: A - B should be minimum.
// The larger the number of bottles, the better.
fn main() {
    input!{n: usize, m: usize, mut ab: [(usize, usize); m]}
    // Minimize the reduction! 減少量を最小化する!
    ab.sort_by_key((|&(origin, new)| origin - new));
    let mut bottles: usize = n;
    let mut count: usize = 0;
    for &(origin, new) in ab.iter() {
        if bottles < origin {
            continue;
        }
        let repeat: usize = (bottles - origin) / (origin - new) + 1;
        count += repeat;
        bottles -= repeat * (origin - new);
    }
    println!("{}", count);
}