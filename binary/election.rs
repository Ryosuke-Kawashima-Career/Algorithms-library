use proconio::input;
// 鉄則C11
// ドント式: 「票数÷議席数」の大きい方から議席を割り当て
// 比例代表制の選挙: 当選できるボーダーを二分探索
// ボーダーラインが分かれば、当選者数がわかる -> 二分探索
fn main() {
    input!{n: usize, k: usize, votes: [f64; n]}

    // 投票数lの方が当選者数が大きくなる
    let mut l: f64 = 1.0;
    let mut r: f64 = 1e9;
    // 合計の議席がk以上になる最大の投票数
    let mut border: f64 = 0.0;
    for _ in 0..100 {
        let mid: f64 = (l + r) / 2.0;
        if calc_elected(mid, &votes) >= k {
            l = mid;
            // 最大のボーダーラインを求める
            border = border.max(mid);
        } else {
            r = mid;
        }
    }

    for i in 0..n {
        let elected: usize = (votes[i] / border) as usize;
        print!("{} ", elected);
    }
}

// ボーダーの何倍票数をとったかが当選者数
fn calc_elected(border: f64, votes: &Vec<f64>) -> usize {
    let mut cnt: usize = 0;
    let n: usize = votes.len();

    for i in 0..n {
        cnt += (votes[i] / border) as usize;
    }

    return cnt;
}