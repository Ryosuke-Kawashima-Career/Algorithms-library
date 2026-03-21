use proconio::input;
// abc338d
// imos法: 差分配列の管理
// vの代わりにその差分配列v'(v'i=vi-vi-1と定義される配列)を管理する
// 「vl,vl+1,…,vrにxを加算する」という操作は「v'lにxを加算しv'r+1に-xを加算する」という操作に置き換わる
fn main() {
    input!{n: usize, m: usize, x: [usize; m]}
    let x: Vec<usize> = x.iter().map(|&xi| xi-1).collect();
    let mut diffs: Vec<i64> = vec![0; n+1];
    let dist = |from: usize, to: usize| -> i64 {
        // 順転する
        if from <= to {
            return (to - from) as i64;
        } else {
            // 反転する
            return (to + n - from) as i64;
        }
    };
    let mut imos_add = |from: usize, to: usize, num: i64| {
        if from <= to {
            diffs[from] += num;
            diffs[to] -= num;
        } else {
            // from -> n -> 0 -> to
            diffs[from] += num;
            diffs[n] -= num;
            diffs[0] += num;
            diffs[to] -= num;
        }
    };

    for i in 0..m-1 {
        imos_add(x[i], x[i+1], dist(x[i+1], x[i]));
        imos_add(x[i+1], x[i], dist(x[i], x[i+1]));
    }
    let mut ans: i64 = 1 << 60;
    for i in 0..n {
        diffs[i+1] += diffs[i];
        ans = ans.min(diffs[i]);
    }
    println!("{}", ans);
}