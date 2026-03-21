use proconio::input;
const INF: usize = 1 << 60;
// ABC440C
// Q. 『 W マス連続して黒く塗り、続けて W マス連続して白いままにする』の 2W 周期でマスを塗る
// Paraphrased: 2W 個のマスが円環上に並んでおり、各マスにはコストが定まっている。連続する W マスを塗るときの、塗るマスのコストの和の最小値は？
// A. まず適当なマスから連続する W マスを塗る際のコストを求めます。その後、尺取法のように「片方の端を1マス伸ばし、もう片方の端を1マス縮める」とすることで、塗る区間を1マスずつずらしたもののコストを O(1) で計算できます。
// A. 尺取り法か累積和
fn main() {
    input! {t: usize}
    let mut answer: Vec<usize> = vec![];
    for _case in 0..t {
        input! {n: usize, width: usize, c: [usize; n]}
        let mut min_cost: usize = INF;
        // Periodicity and Circular Sum
        let period = 2 * width;
        let mut sum_periods: Vec<usize> = vec![0; period];
        for i in 0..n {
            sum_periods[i % period] += c[i];
        }
        let mut prefix_sum_periods: Vec<usize> = vec![0; 2 * period + 1];
        for i in 1..=2 * period {
            prefix_sum_periods[i] = prefix_sum_periods[i - 1] + sum_periods[(i - 1) % period];
        }
        for start in 0..2 * period {
            let end = start + width;
            if end > 2 * period {
                break;
            }
            let cost = prefix_sum_periods[end] - prefix_sum_periods[start];
            min_cost = min_cost.min(cost);
        }
        answer.push(min_cost);
    }
    for a in answer {
        println!("{}", a);
    }
}
