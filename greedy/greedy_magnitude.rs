use proconio::input;
// ABC437C
// Q. here are N reindeer and one sled. The i-th reindeer has weight Wi and strength Pi.
// For each reindeer, choose either "pull the sled" or "ride on the sled". Here, the total strength of the reindeer pulling the sled must be greater than or equal to the total weight of the reindeer riding on the sled. What is the maximum number of reindeer that can ride on the sled?
// A. Greedy Algorithm
// 計算量と制約から、DPが難しいと判断。なので、貪欲法を試みる。
// 貪欲法のカギとなる部分は、power + weightが大きい順に並べる。
fn main() {
    input! {t: usize}
    let mut answer: Vec<usize> = Vec::new();
    for _case in 0..t {
        input! {n: usize, wp: [(i64, i64); n]}
        // power + weight => 良くも悪くも影響力が強いやつから検討する。
        let mut magnitudes: Vec<(i64, usize)> = Vec::new();
        let mut total_weight: i64 = 0;
        for i in 0..n {
            magnitudes.push((wp[i].1 + wp[i].0, i));
            total_weight += wp[i].0;
        }
        magnitudes.sort_by_key(|x| x.0);
        let mut total_power: i64 = 0;
        let mut ans: usize = n;
        for i in (0..n).rev() {
            // 貪欲法のカギとなる部分
            total_weight -= wp[magnitudes[i].1].0;
            total_power += wp[magnitudes[i].1].1;
            ans -= 1;
            if total_power >= total_weight {
                break;
            }
        }
        answer.push(ans);
    }
    for i in 0..t {
        println!("{}", answer[i]);
    }
}
