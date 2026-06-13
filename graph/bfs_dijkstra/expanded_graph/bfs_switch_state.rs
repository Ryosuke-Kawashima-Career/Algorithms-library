use proconio::{input, marker::Chars};
const INF: usize = 1 << 60;
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [
    (N1, 0),
    (0, N1), (0, 1),
    (1, 0)
];
// abc420D
// Q. ?のマスを2回踏むと状態は元に戻ります。
// A. distance[row][column][how many ? cells] を「?のマスを踏んだ回数がmod2でx回である状態でマス(row,column)に到達するまでの操作回数の最小値」とした幅優先探索を行う
fn main() {
    input!{h: usize, w: usize, a: [Chars; h]}
    let (ys, xs, yg, xg) = get_start_end(&a);
    let dist: Vec<Vec<Vec<usize>>> = bfs(ys, xs, &a);
    if dist[yg][xg][0] == INF && dist[yg][xg][1] == INF {
        println!("-1");
    } else {
        println!("{}", dist[yg][xg][0].min(dist[yg][xg][1]));
    }
}

fn bfs(h0: usize, w0: usize, graph: &Vec<Vec<char>>) -> Vec<Vec<Vec<usize>>> {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    // dist[y][x][state(switch)]
    let mut dist: Vec<Vec<Vec<usize>>> = vec![vec![vec![INF; 2]; w]; h];
    dist[h0][w0][0] = 0;
    // (y, x, switched)
    let mut que = std::collections::VecDeque::new();
    que.push_back((h0, w0, 0));

    while let Some((y, x, switched)) = que.pop_front() {
        for &(dy, dx) in D4.iter() {
            let next_y: usize = y.wrapping_add(dy);
            let next_x: usize = x.wrapping_add(dx);
            if next_y < h && next_x < w {
                if graph[next_y][next_x] == '#' {
                    continue;
                }
                if switched % 2 == 0 {
                    if graph[next_y][next_x] == 'x' {
                        continue;
                    }
                } else {
                    if graph[next_y][next_x] == 'o' {
                        continue;
                    }
                }
                if graph[next_y][next_x] == '?' {
                    if dist[next_y][next_x][(switched+1)%2] > dist[y][x][switched] + 1 {
                        dist[next_y][next_x][(switched+1)%2] = dist[y][x][switched] + 1;
                        que.push_back((next_y, next_x, (switched+1)%2));
                    }
                } else {
                    if dist[next_y][next_x][switched] > dist[y][x][switched] + 1 {
                        dist[next_y][next_x][switched] = dist[y][x][switched] + 1;
                        que.push_back((next_y, next_x, switched));
                    }
                }
            }
        }
    }

    return dist;
}

fn get_start_end(graph: &Vec<Vec<char>>) -> (usize, usize, usize, usize) {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    let mut ys = h;
    let mut yg = h;
    let mut xs = w;
    let mut xg = w;
    for i in 0..h {
        for j in 0..w {
            if graph[i][j] == 'S' {
                ys = i;
                xs = j;
            }
            if graph[i][j] == 'G' {
                yg = i;
                xg = j;
            }
        }
    }

    return (ys, xs, yg, xg);
}