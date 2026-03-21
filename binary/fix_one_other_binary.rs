use proconio::input;
// abc321d
// 一つの変数を固定して全探索, もう片方を二分探索
fn main() {
    // price = min(a[i]+b[j], p)
    input!{n: usize, m: usize, p: usize, a: [usize; n], mut b: [usize; m]}
    // sortしてから計算する
    b.sort();
    let mut prefix_b: Vec<usize> = vec![0; m+1];
    // 区間クエリなので累積和
    for i in 1..=m {
        prefix_b[i] = prefix_b[i-1] + b[i-1];
    }
    // aを固定して考える
    let mut sums: Vec<usize> = vec![0; n];

    for i in 0..n {
        // lower_bound
        let index: usize = b.partition_point(|&bj| a[i] + bj < p);
        // 前半(0..index): a + b < p, 後半(index..end): a + b >= p
        sums[i] = a[i] * index + prefix_b[index] + p * (m - index);
    }

    let all_sum: usize = sums.iter().sum();
    println!("{}", all_sum);
}