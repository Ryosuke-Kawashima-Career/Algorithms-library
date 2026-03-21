use proconio::input;
// abc396c
// Q. the number of blacks >= the number of whites
// Choose zero or more balls so that the number of black balls chosen is at least the number of white balls chosen. Among all such choices, find the maximum possible sum of the values of the chosen balls.
// A. greedy algo + prefix
fn main() {
    input!{n: usize, m: usize, mut b: [i64; n], mut w: [i64; m]}
    // sort for greedy
    b.sort_by(|x, y| y.cmp(&x));
    w.sort_by(|x, y| y.cmp(&x));
    let mut prefix_b: Vec<i64> = vec![0; n+1];
    let mut prefix_w: Vec<i64> = vec![0; m+1];
    // get the maximum values beforehand
    let mut prefix_max_w: Vec<i64> = vec![0; m+1];
    for i in 1..=n {
        prefix_b[i] = prefix_b[i-1] + b[i-1];
    }
    for i in 1..=m {
        prefix_w[i] = prefix_w[i-1] + w[i-1];
        prefix_max_w[i] = prefix_max_w[i-1].max(prefix_w[i]);
    }
    let mut ans: i64 = 0;
    for i in 0..=n {
        ans = ans.max(prefix_b[i] + prefix_max_w[i.min(m)]);
    }
    println!("{}", ans);
}