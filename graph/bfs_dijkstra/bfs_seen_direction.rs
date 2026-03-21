use proconio::{input, marker::Chars};
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [
    (0, 1),
    (1, 0),
    (0, N1),
    (N1, 0),
];
fn main() {
    input!{n: usize, m: usize, s: [Chars; n]}
    let ans = bfs((1, 1), &s);
    println!("{}", ans);
}
// abc311d
// seenを3次元にすることで向きの概念を付与する
fn bfs((h0, w0): (usize, usize), graph: &Vec<Vec<char>>) -> usize {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    // seen[i][j][dir: 0..4]
    let mut seen: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; w]; h];
    let mut cnt: usize = 0;
    // (y, x, dir)
    let mut que = std::collections::VecDeque::new();
    for dir in 0..4 {
        que.push_back((h0, w0, dir));
    }

    while let Some((y, x, dir)) = que.pop_front() {
        // (y, x)が未探索か?
        let mut is_yx_seen = false;
        for k in 0..4 {
            if seen[y][x][k] {
                is_yx_seen = true;
            }
        }
        if !is_yx_seen {
            cnt += 1;
        }
        // avoid checking many times!!!
        if seen[y][x][dir] {
            continue;
        }
        seen[y][x][dir] = true;
        let (dy, dx) = D4[dir];
        let next_y: usize = y.wrapping_add(dy);
        let next_x: usize = x.wrapping_add(dx);
        if next_y >= h && next_x >= w {
            continue;
        }
        if graph[next_y][next_x] == '.' {
            if !seen[next_y][next_x][dir] {
                que.push_back((next_y, next_x, dir));
            }
        } else {
            // 向きを変える
            for next_dir in 0..4 {
                if next_dir != dir && !seen[y][x][next_dir] {
                    que.push_back((y, x, next_dir));
                }
            }
        }
    }

    return cnt;
}