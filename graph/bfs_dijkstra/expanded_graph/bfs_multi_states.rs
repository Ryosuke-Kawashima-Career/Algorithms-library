use proconio::{input, marker::Chars};
const INF: usize = 1 << 60;
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [
    (N1, 0),
    (0, N1), (0, 1),
    (1, 0)
];
// abc339D
// 幅優先探索の応用
// 状態遷移をノードとエッジの関係図で表現する
fn main() {
    input!{n: usize, s: [Chars; n]}   
    let (y1, x1, y2, x2) = detect_players(&s);
    // P1(y1, x1)とP2(y2, x2)が(yg, xg)に合流するまでの距離 = dist[yg][xg][yg][xg]
    let dist = bfs((y1, x1), (y2, x2), &s);

    let mut min_dist: usize = INF;
    for i in 0..n {
        for j in 0..n {
            // playerが重なるとき
            min_dist = min_dist.min(dist[i][j][i][j]);
        }
    }

    if min_dist == INF {
        println!("-1");
    } else {
        println!("{}", min_dist);
    }
}

fn bfs((h1, w1): (usize, usize), (h2, w2): (usize, usize), graph: &Vec<Vec<char>>) -> Vec<Vec<Vec<Vec<usize>>>> {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    // 四次元配列で二人の位置を管理する
    // dist[player1 y][player1 x][player2 y][player2 x]
    let mut dist: Vec<Vec<Vec<Vec<usize>>>> = vec![vec![vec![vec![INF; w]; h]; w]; h];
    dist[h1][w1][h2][w2] = 0;
    let mut que = std::collections::VecDeque::new();
    // player1とplayer2の座標を同時に管理する
    que.push_back((h1, w1, h2, w2));

    while let Some((y1, x1, y2, x2)) = que.pop_front() {
        for &(dy, dx) in D4.iter() {
            let (mut next_y1, mut next_x1) = (y1.wrapping_add(dy), x1.wrapping_add(dx));
            let (mut next_y2, mut next_x2) = (y2.wrapping_add(dy), x2.wrapping_add(dx));
            // if NG
            if next_y1 >= h || next_x1 >= w || graph[next_y1][next_x1] == '#' {
                next_y1 = y1;
                next_x1 = x1;
            }
            if next_y2 >= h || next_x2 >= w || graph[next_y2][next_x2] == '#' {
                next_y2 = y2;
                next_x2 = x2;
            }
            /*: equivalent code(if not OK)
            if next_y1 < n && next_x1 < n && graph[next_y1][next_x1] != '#' {
            } else {
                next_y1 = y1;
                next_x1 = x1;
            }
            */

            // update dist
            if dist[next_y1][next_x1][next_y2][next_x2] > dist[y1][x1][y2][x2] + 1 {
                dist[next_y1][next_x1][next_y2][next_x2] = dist[y1][x1][y2][x2] + 1;
                que.push_back((next_y1, next_x1, next_y2, next_x2));
            }
        }
    }

    return dist;
}

fn detect_players(graph: &Vec<Vec<char>>) -> (usize, usize, usize, usize) {
    let n: usize = graph.len();
    let (mut y1, mut x1) = (n, n);
    let (mut y2, mut x2) = (n, n);

    for i in 0..n {
        for j in 0..n {
            if graph[i][j] == 'P' {
                // if not updated yet
                if y1 == n && x1 == n {
                    y1 = i;
                    x1 = j;
                } else {
                    y2 = i;
                    x2 = j;
                }
            }
        }
    }

    return (y1, x1, y2, x2);
}