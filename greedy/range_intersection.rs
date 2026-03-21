use proconio::input;
// abc355D: 貪欲法
// 区間[l r]が共通点を持つ組み合わせの数
// rが小さい順にソートし，貪欲法
fn main() {
    input!{n: usize, mut lr: [(usize, usize); n]}
    // 貪欲法のためのソート
    lr.sort_by_key(|tup| tup.1);
    let mut rights: Vec<usize> = Vec::new();
    let mut ans: usize = 0;
    for i in 0..n {
        let num = rights.partition_point(|&x| x < lr[i].0);
        // left以上のrightの数
        ans += rights.len() - num;
        rights.push(lr[i].1);
    }
    println!("{}", ans);
}