use proconio::input;
// abc463c
// Q. h: height l: leaving time. Who is the tallest at the time=t?
// A. suffix + binary search
fn main() {
    input! {n: usize, hl: [(usize, isize); n], q: usize, tq: [isize; q]}
    let mut suffix_max: Vec<usize> = vec![0; n + 1];
    for i in (0..n).rev() {
        suffix_max[i] = suffix_max[i + 1].max(hl[i].0);
    }
    let mut ans: Vec<usize> = vec![0; q];
    for (i, time) in tq.iter().enumerate() {
        let index: usize = hl.partition_point(|&x| x.1 <= *time);
        ans[i] = suffix_max[index];
    }
    for i in 0..q {
        println!("{}", ans[i]);
    }
}
