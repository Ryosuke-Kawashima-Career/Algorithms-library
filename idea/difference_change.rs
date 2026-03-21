use proconio::input;
// 典型64
// 変化のみに注目する。
// 区間クエリなのであらかじめ前処理をする。
fn main() {
    input!{n: usize, q: usize, a: [i64; n]}
    // 0indexed(0..n-1のindexに意味あり)
    let mut difs: Vec<i64> = vec![0; n-1];
    let mut difs_sum: i64 = 0;
    for i in 0..n-1 {
        difs[i] = a[i+1] - a[i];
        difs_sum += difs[i].abs();
    }

    for _ in 0..q {
        input!{l: usize, r: usize, v: i64}
        // 0indexed
        let (l, r) = (l-1, r-1);
        let mut before_dif: i64 = 0;
        let mut after_dif: i64 = 0;
        // 境界条件で考える: 境界となる値を代入する(l = 1, r = n-2)
        // difsのindexの境界にちょうど入る!!! ex. l-1 = 0, r = n-2
        if l >= 1 {
            before_dif += difs[l-1].abs();
            difs[l-1] += v;
            after_dif += difs[l-1].abs();
        }
        if r < n-1 {
            before_dif += difs[r].abs();
            difs[r] -= v;
            after_dif += difs[r].abs();
        }
        difs_sum += after_dif - before_dif;
        println!("{}", difs_sum);
    }
}