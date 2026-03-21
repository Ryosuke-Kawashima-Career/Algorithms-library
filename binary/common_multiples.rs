use proconio::input;
// 鉄則A12
// Common Multiples
// Q. Printter Ai  outputs paper every a[i]. When will the kth paper be published?
// A. Binary search: Monotonicity
fn main() {
    input!{n: usize, k: usize, a: [usize; n]}
    // 0 paper
    let mut t_l: usize = 0;
    // More than k paper
    let mut t_r: usize = a[0] * k;
    while (t_r - t_l) > 1 {
        let t_med = (t_r + t_l) / 2;
        if count_papers(t_med, &a) >= k {
            t_r = t_med;
        } else {
            t_l = t_med;
        }
    }
    println!("{}", t_r);
}

fn count_papers(t: usize, a: &Vec<usize>) -> usize {
    let mut cnt: usize = 0;
    let n: usize = a.len();
    for i in 0..n {
        cnt += t / a[i];
    }
    cnt
}