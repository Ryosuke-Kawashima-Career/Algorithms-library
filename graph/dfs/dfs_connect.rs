use proconio::input;
use proconio::marker::Chars;
const N1: usize = 1usize.wrapping_neg();
const D8: [(usize, usize); 8] = [
    (N1, N1), (N1, 0), (N1, 1),
    (0, N1), (0, 1),
    (1, N1), (1, 0), (1, 1)
];
// abc325c
// ノードの接続の問題: 1. unionfind, 2.DFS
fn main() {
    input!{h: usize, w: usize, s: [Chars; h]}
    let mut seen: Vec<Vec<bool>> = vec![vec![false; w]; h];
    let mut ans: usize = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' && !seen[i][j] {
                ans += 1;
                dfs(i, j, &s, &mut seen);
            }
        }
    }
    println!("{}", ans);
}

fn dfs(y: usize, x: usize, graph: &Vec<Vec<char>>, seen: &mut Vec<Vec<bool>>) {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    seen[y][x] = true;
    for &(dy, dx) in D8.iter() {
        let next_y = y + dy;
        let next_x = x + dx;
        if next_y < h && next_x < w && graph[next_y][next_x] == '#' {
            if seen[next_y][next_x] {
                continue;
            }
            dfs(next_y, next_x, graph, seen);
        }
    }
}