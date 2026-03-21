use proconio::input;
// abc416d
// Q. ∑((Ai+Bi) mod M) としてありうる最小値
// A. Greedy + 尺取り法
fn main() {
    input!{number_of_tests: usize}
    let mut answer: Vec<usize> = Vec::new();
    for _case in 0..number_of_tests {
        input!{n: usize, m: usize, mut a: [usize; n], mut b: [usize; n]}
        let sum_a: usize = a.iter().sum();
        let sum_b: usize = b.iter().sum();
        // The larger A, the better
        a.sort_by(|x, y| y.cmp(&x));
        // The smaller B, the better
        b.sort();
        // Make as many (>=m) as possible
        let mut count_m: usize = 0;
        let mut index: usize = 0;
        // Apply the algo to B
        for i in 0..n {
            while index < n && a[i] + b[index] < m {
                index += 1;
            }
            if index >= n {
                break;
            }
            count_m += 1;
            index += 1;
        }
        let ans = sum_a + sum_b - m * count_m;
        println!("{}", ans);
    }
}