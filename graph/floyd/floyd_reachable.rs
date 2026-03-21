use proconio::{input, marker::{Chars, Usize1}};
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [
    (N1, 0),
    (0, N1), (0, 1),
    (1, 0)
];
const INF: usize = 1 << 60;
// abc348D
// 薬を何度でも使えるとしても、2回以上使うことがあれば最後に使った後の行動を代わりに初めて使った後にすればよい
// N頂点の有向グラフであって、「iからjに辺がある」⟺「薬iを使った後他の薬を使わずに薬jの位置にたどり着ける」グラフを構築する
// 薬iを使った後他の薬を使わずに薬jの位置にたどり着けるか?という問題は、グリッド上で薬iの位置からBFSで最短距離を求めればよい
// N個の薬についてその位置からグリッド上で BFS を行うので、計算量はO(HWN)
// ゴール地点に薬がない場合、薬N+1をゴール地点に作ると簡潔
// iからjに辺があるならiからjへの距離は0とする. そうでないなら、正の適当な数字を距離とすればよい
// ワーシャルフロイド法を適用すると、スタートからゴールまでの距離が0ならたどり着ける
fn main() {
    input!{h: usize, w: usize, a: [Chars; h], n: usize, rce: [(Usize1, Usize1, usize); n]}
    let (start_h, start_w, end_h, end_w) = detect_state(&a);
    // 薬の座標にスタート地点とゴール地点を追加する
    let mut potions: Vec<(usize, usize, usize)> = Vec::new();
    potions.push((start_h, start_w, 0));
    for i in 0..n {
        potions.push(rce[i]);
    }
    potions.push((end_h, end_w, 0));

    // is_reachables[i][j]: 薬iからjまでの到達可能か
    let n_potion = n + 2;
    let mut is_reachables: Vec<Vec<bool>> = vec![vec![false; n_potion]; n_potion];
    for i in 0..n_potion {
        let (_, _, energy) = potions[i];
        // dist[yj][xj]: 薬i(yi, xi)からj(yj, xj)までの距離
        let dist: Vec<Vec<usize>> = bfs_dist(i, &potions, &a);
        for j in 0..n_potion {
            let (yj, xj, _) = potions[j];
            if dist[yj][xj] <= energy {
                is_reachables[i][j] = true;
            }
        }
    }

    // distance = 0: reachable, 1: not reachable
    let reachables_floyd: Vec<Vec<usize>> = floyd(&is_reachables);
    // fx,yを「xからyにたどり着けるなら1, そうでないなら0」として、「fi,jをfi,k AND fk,jとのORで置き換える」
    let reachables_floyd_bool: Vec<Vec<bool>> = floyd_bool(&is_reachables);
    if reachables_floyd[0][n_potion-1] == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn floyd(is_reachables: &Vec<Vec<bool>>) -> Vec<Vec<usize>> {
    let n: usize = is_reachables.len();
    // distance = 0: reachable, 1: not reachable
    let mut dist: Vec<Vec<usize>> = vec![vec![1; n]; n];
    for v in 0..n {
        for next in 0..n {
            if is_reachables[v][next] {
                dist[v][next] = 0;
            }
        }
    }

    // ワーシャルフロイド法(DP)
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }

    return dist;
}
fn floyd_bool(is_reachables: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let n: usize = is_reachables.len();
    let mut dp: Vec<Vec<bool>> = is_reachables.clone();

    // ワーシャルフロイド法(DP)
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dp[i][j] |= dp[i][k] & dp[k][j];
            }
        }
    }

    return dp;
}

fn bfs_dist(start_idx: usize, potions: &Vec<(usize, usize, usize)>, graph: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    let (h0, w0, _) = potions[start_idx];
    let mut dist: Vec<Vec<usize>> = vec![vec![INF; w]; h];
    dist[h0][w0] = 0;
    // corner case
    if graph[h0][w0] == '#' {
        return dist;
    }
    let mut que = std::collections::VecDeque::new();
    que.push_back((h0, w0));

    while let Some((y, x)) = que.pop_front() {
        for &(dy, dx) in D4.iter() {
            let next_y: usize = y + dy;
            let next_x: usize = x + dx;
            if next_y < h && next_x < w && graph[next_y][next_x] != '#' {
                if dist[next_y][next_x] > dist[y][x] + 1 {
                    dist[next_y][next_x] = dist[y][x] + 1;
                    que.push_back((next_y, next_x));
                }
            }
        }
    }

    return dist;
}

fn detect_state(graph: &Vec<Vec<char>>) -> (usize, usize, usize, usize) {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    let mut start_h: usize = h;
    let mut start_w: usize = w;
    let mut end_h: usize = h;
    let mut end_w: usize = w;
    for i in 0..h {
        for j in 0..w {
            if graph[i][j] == 'S' {
                start_h = i;
                start_w = j;
            }
            if graph[i][j] == 'T' {
                end_h = i;
                end_w = j;
            }
        }
    }
    return (start_h, start_w, end_h, end_w);
}