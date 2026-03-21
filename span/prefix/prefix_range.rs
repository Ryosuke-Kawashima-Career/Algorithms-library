use proconio::input;
// abc106d
// nが小さいことに注目する
// pi≤LjとRj≤qi
fn main() {
    input!{n: usize, m: usize, query: usize}
    //  区間[l 1], [l 2], ..., [l r]における列車の数 = ranges[l][r]
    let mut ranges: Vec<Vec<usize>> = vec![vec![0; n+1]; n+1];
    for _ in 0..m {
        input!{l: usize, r: usize}
        ranges[l][r] += 1;
    }
    // prefix
    for l in 1..=n {
        // r = l+1..=n
        for r in 2..=n {
            // l: 固定(lから始まるrangeの累積和)
            ranges[l][r] += ranges[l][r-1];
        }
    }
    // 左端を全探索し、右端をprefixで高速に計算する
    for _ in 0..query {
        input!{p: usize, q: usize}
        let mut ans: usize = 0;
        for l in p..=q {
            // r: 固定
            ans += ranges[l][q];
        }
        println!("{}", ans);
    }
}