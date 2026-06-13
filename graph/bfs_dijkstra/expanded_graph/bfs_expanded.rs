use proconio::{input, marker::{Chars, Usize1}};
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [
    (N1, 0),
    (0, N1), (0, 1),
    (1, 0)
];
const INF: usize = 1 << 60;
// abc348D
// 頂点倍加: 同じ頂点でも状態が違うとき異なる頂点とみなすこと(ex. 拡張ダイクストラ)
// 薬を何度でも使えるとしても、2回以上使うことがあれば最後に使った後の行動を代わりに初めて使った後にすればよい
// N頂点の有向グラフであって、「iからjに辺がある」⟺「薬iを使った後他の薬を使わずに薬jの位置にたどり着ける」グラフを構築する
// 薬iを使った後他の薬を使わずに薬jの位置にたどり着けるか?という問題は、グリッド上で薬iの位置からBFSで最短距離を求めればよい
// N個の薬についてその位置からグリッド上で BFS を行うので、計算量はO(HWN)
// ゴール地点に薬がない場合、薬N+1をゴール地点に作ると簡潔
fn main() {
    input!{h: usize, w: usize, a: [Chars; h], n: usize, rce: [(Usize1, Usize1, i64); n]}
    let (start_h, start_w, end_h, end_w) = detect_state(&a);
    // (i, j)の薬のエネルギー
    let mut energies: Vec<Vec<i64>> = vec![vec![0; w]; h];
    for &(r, c, e) in rce.iter() {
        energies[r][c] = e;
    }

    let max_enegies: Vec<Vec<i64>> = bfs_dp((start_h, start_w), &a, &energies);
    if max_enegies[end_h][end_w] != -1 {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn bfs_dp((h0, w0): (usize, usize), graph: &Vec<Vec<char>>, energies: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    // 現在のエネルギー: dist[i][j][energy](頂点倍加)
    let mut dp: Vec<Vec<i64>> = vec![vec![-1; w]; h];
    dp[h0][w0] = 0;
    let mut que = std::collections::VecDeque::new();
    que.push_back((h0, w0));

    while let Some((y, x)) = que.pop_front() {
        for &(dy, dx) in D4.iter() {
            let next_y: usize = y + dy;
            let next_x: usize = x + dx;
            let energy = dp[y][x].max(energies[y][x]);
            if next_y < h && next_x < w && graph[next_y][next_x] != '#' {
                // エネルギーは大きいほどよい!
                if energy >= 1 && dp[next_y][next_x] < energy - 1 {
                    dp[next_y][next_x] = energy - 1;
                    que.push_back((next_y, next_x));
                }
            }
        }
    }

    return dp;
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