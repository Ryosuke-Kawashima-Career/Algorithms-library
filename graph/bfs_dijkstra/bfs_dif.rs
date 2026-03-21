use proconio::input;
use proconio::marker::Chars;
// 鉄則B63
const INF: usize = 1 << 60;
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [
    (N1, 0),
    (0, N1), (0, 1),
    (1, 0)
];
fn bfs(h0: usize, w0: usize, graph: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    let mut dist: Vec<Vec<usize>> = vec![vec![INF; w]; h];
    dist[h0][w0] = 0;
    let mut que = std::collections::VecDeque::new();
    que.push_back((h0, w0));

    while let Some((y, x)) = que.pop_front() {
        for &(dy, dx) in D4.iter() {
            let next_y: usize = y + dy;
            let next_x: usize = x + dx;
            if next_y < h && next_x < w && graph[next_y][next_x] == '.' {
                if dist[next_y][next_x] > dist[y][x] + 1 {
                    dist[next_y][next_x] = dist[y][x] + 1;
                    que.push_back((next_y, next_x));
                }
            }
        }
    }

    return dist;
}

fn bfs((h0, w0): (usize, usize), graph: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    let mut dist: Vec<Vec<usize>> = vec![vec![INF; w]; h];
    dist[h0][w0] = 0;
    let mut que = std::collections::VecDeque::new();
    que.push_back((h0, w0));

    while let Some((y, x)) = que.pop_front() {
        for &(dy, dx) in D4.iter() {
            let next_y: usize = y + dy;
            let next_x: usize = x + dx;
            if next_y < h && next_x < w && graph[next_y][next_x] != 'X' {
                if dist[next_y][next_x] > dist[y][x] + 1 {
                    dist[next_y][next_x] = dist[y][x] + 1;
                    que.push_back((next_y, next_x));
                }
            }
        }
    }

    return dist;
}

// abc088d
// 余事象と発想が似ている
fn main() {
    input!{h: usize, w: usize, graph: [Chars; h]}
    let dif: Vec<(i64, i64)> = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
    let mut distance: Vec<Vec<usize>> = vec![vec![INF; w]; h];
    distance[0][0] = 0;
    let mut que = std::collections::VecDeque::new();
    que.push_back((0, 0));

    while let Some((i, j)) = que.pop_front() {
        for &(y, x) in dif.iter() {
            if 0 <= (i as i64 + y) && (i as i64 + y) < h as i64
            && 0 <= (j as i64 + x) && (j as i64 + x) < w as i64 
            && graph[i + y as usize][j + x as usize] == '.' 
            && distance[i + y as usize][j + x as usize] > distance[i][j] + 1 {
                distance[i + y as usize][j + x as usize] = distance[i][j] + 1;
                que.push_back((i + y as usize, j + x as usize));
            }
        }
    }
    let mut num_of_black: usize = 0;
    for i in 0..h {
        for j in 0..w {
            if graph[i][j] == '#' {
                num_of_black += 1;
            }
        }
    }
    let num_of_path_blocks: usize = distance[h-1][w-1] + 1;
    let ans: usize = (h * w) - num_of_path_blocks - num_of_black;

    if distance[h-1][w-1] == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}