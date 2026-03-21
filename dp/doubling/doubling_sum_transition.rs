use proconio::{input, marker::Usize1};
// abc438-e
// Q. Pass the bucket of the person(i) to the person(a[i]). The operations are more than 10^9!!!
// A. Doubling(Dynamic Programming)
// A. functional graph
/*: Key Points
- ノードが閉路に含まれている場合は、その閉路内のノードにしか辿り着けない
- 閉路外にノードがある場合は、閉路内のノードと閉路に辿り着くまでのパス上にあるノードに辿り着くことができる */
fn main() {
    input! {n: usize, q: usize, a: [Usize1; n], tb: [(u64, Usize1); q]}
    // Doubling configuration
    let log_k = 32;
    // next_person[k][i] = where you end up after 2^k steps starting from i = if k == 0, next_person[0][i] = a[i]
    let mut next_person: Vec<Vec<usize>> = vec![vec![0; n]; log_k];
    let mut sum: Vec<Vec<u64>> = vec![vec![0; n]; log_k];
    // Initialization for k=0 (1 step)
    for person in 0..n {
        next_person[0][person] = a[person];
        sum[0][person] = (person + 1) as u64;
    }
    // Build the doubling tables by Recurrence Formula
    for k in 1..log_k {
        for person in 0..n {
            next_person[k][person] = next_person[k - 1][next_person[k - 1][person]];
            // The sum for 2^(k+1) steps is sum of first 2^k steps + sum of next_person 2^k steps
            // sumではなくpersonを因数にとることに注意∵sum[logK][person]
            sum[k][person] = sum[k - 1][person] + sum[k - 1][next_person[k - 1][person]];
        }
    }
    // Answer queries
    for (t, b) in tb {
        let mut cur_person: usize = b;
        let mut total_water: u64 = 0;
        // 後ろから逆算する
        for k in (0..log_k).rev() {
            if (t >> k) & 1 == 1 {
                total_water += sum[k][cur_person];
                cur_person = next_person[k][cur_person];
            }
        }
        println!("{}", total_water);
    }
}
