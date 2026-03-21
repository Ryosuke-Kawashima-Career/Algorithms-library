use proconio::input;
// 鉄則A10
// 左右からその時点の最大値を記録する。(1indexed)
// 二方向から挟み撃ち: -> O <-
// 区間外([l r])の最大値がわかる。
// segmentTree is also OK
fn main() {
    input!{n: usize, a: [i64; n], d: usize}
    let mut prefix_max_l: Vec<i64> = vec![0; n+2];
    let mut prefix_max_r: Vec<i64> = vec![0; n+2];

    for i in 1..=n {
        prefix_max_l[i] = prefix_max_l[i-1].max(a[i-1]);
        // 添え字に気を付ける。
        prefix_max_r[n-i+1] = prefix_max_r[n-i+2].max(a[n-i]);
        /* for文を反対から回す方法もある
        for i in (1..=n).rev() {
            max_right[i] = max_right[i+1].max(a[i-1]);
        } 
        */
    }

    for _ in 0..d {
        input!{l: usize, r: usize}
        let max_l: i64 = prefix_max_l[l-1];
        let max_r: i64 = prefix_max_r[r+1];
        let max_lr: i64 = max_l.max(max_r);
        println!("{}", max_lr);
    }
}