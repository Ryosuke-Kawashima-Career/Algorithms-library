use proconio::input;
const INF: i64 = 1 << 60;
// abc074d
// ワーシャルフロイド法と最小値を達成する辺を選ぶ
fn main() {
    input!{n: usize, mut a: [[i64; n]; n]}
    let mut ans: i64 = 0;
    let mut edges: Vec<(usize, usize, i64)> = Vec::new();
    // i, jのcombination
    for i in 0..n {
        for j in i+1..n {
            edges.push((i, j, a[i][j]));
            ans += a[i][j];
        }
    }
    // 辺の大きい順にsort
    edges.sort_by(|tup1, tup2| tup2.2.cmp(&tup1.2));

    // 辺の大きさ順にワーシャルフロイド法
    for &(i, j, cost) in edges.iter() {
        // k: medium
        for k in 0..n {
            if k != i && k != j && a[i][k] != INF && a[k][j] != INF {
                // 不要な辺を消去する(kが中継点になるのでi,jを直接結ぶ道は要らない)
                if a[i][k] + a[k][j] == cost {
                    ans -= cost;
                    a[i][j] = INF;
                    a[j][i] = INF;
                    break;
                } else if a[i][k] + a[k][j] < cost {
                    // さらに更新できるとき
                    println!("-1");
                    return;
                }
            }
        }
    }
    println!("{}", ans);
}