use proconio::input;
use superslice::Ext;
// JOI2008C ダーツ
// 半分全列挙 + 二分探索
// 必ずしも選ぶ数を半分にする必要はない
fn main() {
    input!{n: usize, m: usize, mut p: [usize; n]}
    // pから4つまで選ぶ(score <= m)
    let mut scores: Vec<usize> = Vec::new();
    // 0回
    scores.push(0);
    // 1回
    for i in 0..n {
        scores.push(p[i]);
    }
    // 2回
    for i in 0..n {
        for j in 0..n {
            scores.push(p[i] + p[j]);
        }
    }
    scores.sort();

    let mut max_score: usize = 0;
    for i in 0..scores.len() {
        let score: usize = scores[i];
        if score <= m {
            let index: usize = scores.lower_bound(&(m - score));
            if index >= 1 && score + scores[index - 1] <= m {
                max_score = max_score.max(score + scores[index-1]);
            }
        }
    }

    println!("{}", max_score);
}