use proconio::{input, marker::Usize1};
// JOI 2015 本選 1 - 鉄道運賃
// imos法で線路を使う頻度を計算する
fn main() {
    input!{n: usize, m: usize, p: [Usize1; m], abc: [(i64, i64, i64); n-1]}
    // imos法を使ってエッジの貢献度を調べる
    let mut imos: Vec<i64> = vec![0; n];
    for i in 1..m {
        imos[p[i-1].min(p[i])] += 1;
        imos[p[i-1].max(p[i])] -= 1;
    }
    for i in 1..n {
        imos[i] += imos[i-1];
    }

    let mut cost: i64 = 0;
    for i in 0..n-1 {
        // a: 切符の料金, b: 定期料金, c: 定期の値段
        let (a, b, c) = abc[i];
        let ticket: i64 = a * imos[i];
        let card: i64 = b * imos[i] + c;
        cost += ticket.min(card);
    }
    println!("{}", cost);
}