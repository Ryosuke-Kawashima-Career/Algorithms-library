use proconio::{input, marker::Chars};
const INF: usize = 1 << 60;
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [
    (N1, 0),
    (0, N1),
    (1, 0),
    (0, 1),
];
// abc387d
// Q. move vertically and horizontally one after another
// A. dist[y-axis][x-axis][direction]
fn main() {
    input!{h: usize, w: usize, s: [Chars; h]}
    let (ys, xs, yg, xg) = positions(&s);
    let dist = bfs(ys, xs, &s);
    let mut ans: usize = INF;
    for dir in 0..4 {
        if dist[yg][xg][dir%2] != INF {
            ans = ans.min(dist[yg][xg][dir%2]);
        }
    }
    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

fn bfs(h0: usize, w0: usize, graph: &Vec<Vec<char>>) -> Vec<Vec<Vec<usize>>> {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    // dist[y][x][dir: 0: vertical, 1: horizontal]
    let mut dist: Vec<Vec<Vec<usize>>> = vec![vec![vec![INF; 2]; w]; h];
    for dir in 0..4 {
        dist[h0][w0][dir%2] = 0;
    }
    let mut que = std::collections::VecDeque::new();
    for dir in 0..4 {
        que.push_back((h0, w0, dir));
    }

    while let Some((y, x, dir)) = que.pop_front() {
        for dir_next in 0..4 {
            let (dy, dx) = D4[dir_next];
            let next_y: usize = y.wrapping_add(dy);
            let next_x: usize = x.wrapping_add(dx);
            if next_y < h && next_x < w && graph[next_y][next_x] != '#' {
                if dist[next_y][next_x][dir_next % 2] > dist[y][x][dir % 2] + 1
                && dir % 2 != dir_next % 2 {
                    dist[next_y][next_x][dir_next % 2] = dist[y][x][dir % 2] + 1;
                    que.push_back((next_y, next_x, dir_next));
                }
            }
        }
    }

    return dist;
}

fn positions(graph: &Vec<Vec<char>>) -> (usize, usize, usize, usize) {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    let mut ys: usize = h;
    let mut xs: usize = w;
    let mut yg: usize = h;
    let mut xg: usize = w;
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