use proconio::input;
// 全国統一プログラミング王決定戦本戦 A - Abundant Resources
fn main() {
    input!{n: usize, a: [usize; n]}
    let mut prefix_sums: Vec<usize> = vec![0; n+1];
    // 1indexed
    for i in 1..=n {
        prefix_sums[i] = prefix_sums[i-1] + a[i-1];
    }
    // k: span
    for k in 1..=n {
        let mut max_reserve: usize = 0;
        // use index0 to calc
        for start in 0..=n {
            if k + start <= n {
                let candidate: usize = prefix_sums[k+start] - prefix_sums[start];
                max_reserve = max_reserve.max(candidate);
            }
        }
        println!("{}", max_reserve);
    }
}