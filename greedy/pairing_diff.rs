use proconio::input;
// abc334c
// 二つの数値のペア
// 1. なくしていない色の靴下はその色同士で組を作るのが最適
// 2. K が偶数のとき、(A1,A2),(A3,A 4),…,(Ak-1,Ak)というように
// (昇順に並べた列内で)隣接する色同士をペアにしていくのが最適
// 3. 先頭から順に組にして行ったときの奇妙さの総和を累積和として求め、末尾からの累積和も同様に求めておく
fn main() {
    // a: ソート済み
    input!{n: usize, k: usize, a: [usize; k]}
    // 1indexed
    let mut prefix_left: Vec<usize> = vec![0; k+1];
    let mut prefix_right: Vec<usize> = vec![0; k+1];
    // 左から差の累積を取る
    for i in 1..=k {
        prefix_left[i] = prefix_left[i-1];
        if i % 2 == 0 {
            prefix_left[i] += a[i-1] - a[i-2];
        }
    }
    // 右の場合
    for i in (0..k).rev() {
        prefix_right[i] = prefix_right[i+1];
        if (k - i) % 2 == 0 {
            prefix_right[i] += a[i+1] - a[i];
        }
    }
    
    let mut ans: usize = 1 << 60;
    // A1,A3,A5,…,Akだけを探索
    for i in (0..=k).step_by(2) {
        ans = ans.min(prefix_left[i] + prefix_right[i]);
    }
    println!("{}", ans);
}