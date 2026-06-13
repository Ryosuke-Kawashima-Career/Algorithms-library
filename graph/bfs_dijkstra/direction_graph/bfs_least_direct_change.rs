use proconio::{input, marker::Chars};
const INF: usize = 1 << 60;
const N1: usize = 1usize.wrapping_neg();
// ノードの向きを表す。
const D4: [(usize, usize); 4] = [
    (N1, 0),
    (0, N1), (0, 1),
    (1, 0)
];
// 典型43
// 拡張ダイクストラ(ノードに向きの概念を追加)
// 距離の定義の仕方: 方向転換一回を距離1と定義する．
// 頂点の定義: i * w + j
// i: node / w, j: node % w
fn main() {
    input!{h: usize, w: usize, rs: usize, cs: usize, rt: usize, ct: usize, s: [Chars; h]}
    let (rs, cs, rt, ct) = (rs-1, cs-1, rt-1, ct-1);

    let min_change_directions = bfs_direct(rs, cs, &s);
    let min_change_direction: usize = *min_change_directions[rt][ct].iter().min().unwrap();
    println!("{}", min_change_direction);
}

fn bfs_direct(start_h: usize, start_w: usize, graph: &Vec<Vec<char>>) -> Vec<Vec<Vec<usize>>> {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    // 距離にもう一つの状態を記録する軸を設定する．
    // (rs, cs)からの距離 = dist[i: row][j: col][k: direction]
    let mut dist: Vec<Vec<Vec<usize>>> = vec![vec![vec![INF; 4]; w]; h];
    // 現在地と進行方向を記録する
    let mut que = std::collections::VecDeque::new();
    for k in 0..4 {
        dist[start_h][start_w][k] = 0;
        let (dy, dx) = D4[k];
        if start_h + dy < h && start_w + dx < w 
        && graph[start_h + dy][start_w + dx] == '.' {
            que.push_back((start_h, start_w, k));
        }
    }

    while let Some((y, x, direct)) = que.pop_front() {
        // 隣接点を調べる。
        for k in 0..4 {
            let (dy, dx) = D4[k];
            if y + dy < h && x + dx < w 
            && graph[y + dy][x + dx] == '.' {
                // 進行方向が同じとき
                if k == direct {
                    if dist[y+dy][x+dx][k] > dist[y][x][direct] {
                        dist[y+dy][x+dx][k] = dist[y][x][direct];
                        que.push_back((y+dy, x+dx, k));
                    }
                } else {
                    if dist[y+dy][x+dx][k] > dist[y][x][direct] + 1 {
                        dist[y+dy][x+dx][k] = dist[y][x][direct] + 1;
                        que.push_back((y+dy, x+dx, k));
                    }
                }
            }
        }
    }

    return dist;
}