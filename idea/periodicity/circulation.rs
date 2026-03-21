use proconio::input;
// abc384d
// Q. 周期Nをもつ無限数列A=(A1,A2,A3,…)の先頭N項A1,A2,…,ANが与えられます。この数列の空でない連続する部分列のうち、和がSとなるものが存在するか
// A. 2倍の長さ(循環を表現)の累積和と二分探索
fn main() {
    input!{n: usize, s: i64, a: [i64; n]}
    let mut prefix: Vec<i64> = vec![0; 2*n+1];
    let sum_a: i64 = a.iter().sum();
    for i in 1..=2*n {
        prefix[i] = prefix[i-1] + a[(i-1) % n];
    }

    if s % sum_a == 0 {
        println!("Yes");
        return;
    }
    let remain = s % sum_a;
    for i in 0..=2*n {
        // lowerbound
        let index: usize = prefix.partition_point(|&x| x < remain + prefix[i]);
        // avoid index out of bounds and check the equality
        if index <= 2 * n && prefix[i] + remain == prefix[index] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}