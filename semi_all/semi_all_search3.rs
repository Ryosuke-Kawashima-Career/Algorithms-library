use proconio::input;
use superslice::Ext;
// 典型51
// 半分全列挙
// ちょうどk個えらんでp以下にする。
fn main() {
    input!{n: usize, k: usize, p: i64, a: [i64; n]}
    let n1: usize = n / 2;
    let n2: usize = n - n1;
    // score = scores[i: num of items][j: sorted index]
    let mut scores1: Vec<Vec<i64>> = vec![vec![]; n1+1];
    let mut scores2: Vec<Vec<i64>> = vec![vec![]; n2+1];

    enum_scores(n1, &a[..n1], &mut scores1);
    enum_scores(n2, &a[n1..], &mut scores2);

    // sort
    for i in 0..=n1 {
        scores1[i].sort();
    }
    for i in 0..=n2 {
        scores2[i].sort();
    }

    let mut ans: usize = 0;
    // 使った品物の数: k
    for k1 in 0..=k {
        let k2: usize = k - k1;
        for &score1 in scores1[k1].iter() {
            // p - score1 以下の数
            let index: usize = scores2[k2].upper_bound(&(p - score1));
            ans += index;
        }
    }

    println!("{}", ans);
}

fn enum_scores(n: usize, prices: &[i64], scores: &mut Vec<Vec<i64>>) {
    for bit in 0..(1 << n) {
        let mut score: i64 = 0;
        let mut used: usize = 0;
        for i in 0..n {
            if bit >> i & 1 == 1 {
                score += prices[i];
                used += 1;
            }
        }
        scores[used].push(score);
    }
}