use proconio::input;
// arc100D
// 変数がいくつかある時は、同時に考えるのではなく、あるものを固定して考えるとうまくいく
// ２変数あって全探索すると計算量がO(N^2)となる場合は、１つを固定して考えると、もう片方を工夫によって高速に求められ、計算量が落ちる
// 3つのものを考える時は、真ん中の物を固定すると上手くいく
// 4つのものは3つの仕切りがあると考えることもできるので、2つずつに分けて捉えるとうまくいくかも
fn main() {
    input!{n: usize, a: [i64; n]}
    let mut prefix: Vec<i64> = vec![0; n+1];
    for i in 1..=n {
        prefix[i] = prefix[i-1] + a[i-1];
    }
    let mut min_diff: i64 = 1 << 60;
    // aを4つに分ける: 0..i1, i1..i2, i2..i3, i3..n -> 仕切りは3つ
    // 真ん中のi2を固定する
    for i2 in 2..=n-2 {
        // 4つの部分列の中に空集合は存在しない!
        // 割り算を掛け算に変換し，計算の誤差を消す
        let p1 = prefix[1..i2].partition_point(|&sum| 2 * sum < prefix[i2]);
        // i2+1に注意する
        let p3 = prefix[i2+1..n].partition_point(|&sum| 2 * sum < prefix[n] + prefix[i2]) + i2;
        for i1 in p1..=p1+1 {
            for i3 in p3..=p3+1 {
                if i1 <= n && i3 <= n {
                    let sum1 = prefix[i1] - prefix[0];
                    let sum2 = prefix[i2] - prefix[i1];
                    let sum3 = prefix[i3] - prefix[i2];
                    let sum4 = prefix[n] - prefix[i3];
                    let max_sum = sum1.max(sum2).max(sum3).max(sum4);
                    let min_sum = sum1.min(sum2).min(sum3).min(sum4);
                    min_diff = min_diff.min(max_sum - min_sum);
                }
            }
        }
    }

    println!("{}", min_diff);
}