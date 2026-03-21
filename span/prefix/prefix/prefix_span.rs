use proconio::input;
// 全国統一プログラミング王決定戦本戦 A - Abundant Resources
// 区間クエリ: 累積和
fn main() {
    input!{n: usize, a: [i64; n]}
    let mut prefix: Vec<i64> = vec![0; n+1];
    for i in 1..=n {
        prefix[i] = prefix[i-1] + a[i-1];
    }
    // 区間DPの実装のパクリ
    for span in 1..=n {
        let mut max_val: i64 = 0;
        for start in 0..n {
            let end: usize = start + span;
            if end > n {
                continue;
            }
            let val: i64 = prefix[end] - prefix[start];
            max_val = max_val.max(val);
        }
        println!("{}", max_val);
    }
}