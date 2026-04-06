use proconio::input;
const INF: i64 = 1 << 60;
// 典型87
// 組み合わせの数がxについて単調減少 -> 二分探索
// x: 道の長さ, p: 街iからjまでの距離の限界, k: 街の組み合わせ, n: 街の数
// graph: 隣接行列
fn main() {
    input!{n: usize, p: i64, k: usize, graph: [[i64; n]; n]}
    // 組み合わせがk以上になる境界
    let lower_bound_x: i64 = get_border(k, p, &graph);
    // 組み合わせがk未満になる境界
    let upper_bound_x: i64 = get_border(k-1, p, &graph);

    if count_combinations(INF, p, &graph) == k {
        println!("Infinity");
    } else {
        println!("{}", upper_bound_x - lower_bound_x);
    }
}

fn get_border(k: usize, p: i64, graph: &Vec<Vec<i64>>) -> i64 {
    // 組み合わせがk以下になる境界
    let mut lx: i64 = 1;
    let mut rx: i64 = INF;
    let mut border: i64 = INF;
    // for文で実装する!!!
    for _ in 0..100 {
        let midx: i64 = (lx + rx) / 2;
        if count_combinations(midx, p, graph) <= k {
            rx = midx;
            // 最小となる境界を求める!!!
            border = border.min(midx);
        } else {
            lx = midx;
        }
    }

    // rxでもよい(borderを実装する意味?)
    return border;
}

// ワーシャルフロイド法でdpを使ってi,j間の距離を計算する
fn count_combinations(x: i64, p: i64, graph: &Vec<Vec<i64>>) -> usize {
    let mut combination: usize = 0;
    let n: usize = graph.len();
    let mut dist: Vec<Vec<i64>> = vec![vec![INF; n]; n];
    for i in 0..n {
        for j in 0..n {
            if graph[i][j] == -1 {
                dist[i][j] = x;
            } else {
                dist[i][j] = graph[i][j];
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }

    // combinationなのでj = i+1..n
    for i in 0..n {
        for j in (i+1)..n {
            if dist[i][j] <= p {
                combination += 1;
            }
        }
    }

    return combination;
}