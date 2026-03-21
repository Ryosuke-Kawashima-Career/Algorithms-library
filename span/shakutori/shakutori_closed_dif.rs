use proconio::input;
// 鉄則A13
// 尺取り法: 条件を満たすような区間をすべて求める方法
fn main() {
    input!{n: usize, k: i64, a: [i64; n]}
    let mut index: Vec<usize> = vec![0; n];
    for i in 0..n {
        if i > 0 {
            index[i] = index[i-1];
        }
        
        // ギリギリまで増やす。indexは条件を満たす限界で終わる。
        while index[i] + 1 < n && a[index[i] + 1] - a[i] <= k {
            index[i] += 1;
        }
    }

    let mut ans: usize = 0;
    for i in 0..n {
        // 異なる二つのペアを見つける。
        ans += index[i] - i;
    }
    println!("{}", ans);
}