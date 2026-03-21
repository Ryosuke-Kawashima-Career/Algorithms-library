use proconio::input;
// 鉄則90A38
// 上界を見積もる。
// 上界が達成される場合がどのような場合化を考える。
fn main() {
    input!{d: usize, n: usize}
    let mut limit: Vec<usize> = vec![24; d];
    for _ in 0..n {
        input!{l: usize, r: usize, h: usize}
        let (l, r) = (l-1, r-1);
        for i in l..=r {
            limit[i] = limit[i].min(h);
        }
    }
    let ans: usize = limit.iter().sum::<usize>();
    println!("{}", ans);
}