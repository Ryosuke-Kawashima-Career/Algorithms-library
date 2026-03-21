use proconio::{input, marker::Chars};
use std::collections::VecDeque;
const INF: usize = 1 << 60;
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [(N1, 0), (0, N1), (0, 1), (1, 0)];
// ABC436D
// Q. Given a grid with walls (#), open cells (.), and teleport cells (a-z), find the shortest path from the top-left to the bottom-right cell, using teleports optimally.
// A. Build a graph representing the grid, including edges for adjacent open cells and teleport connections. Then perform a BFS to find the shortest path.
// BFS の過程においてあるワープマスを訪れたとする。もし、このワープマスと同じ文字が書かれた別のワープマスに既に訪れたことがあるのであれば、今訪れているマスからのワープについては考える必要がない。
// なぜならば、今訪れているマスからワープをするよりも、既に訪れた同じ文字のワープマスからワープをした方が得だから.
fn main() {
    input! {h: usize, w: usize, s: [Chars; h]}
    let mut teleports: Vec<Vec<usize>> = vec![vec![]; 26];

    for r in 0..h {
        for c in 0..w {
            let ch = s[r][c];
            if ch.is_ascii_lowercase() {
                teleports[ch as usize - 'a' as usize].push(r * w + c);
            }
        }
    }
    let dist = bfs_teleport(0, &s, &teleports);
    let ans = dist[h * w - 1];
    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

fn bfs_teleport(start: usize, graph: &Vec<Vec<char>>, teleports: &Vec<Vec<usize>>) -> Vec<usize> {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    let mut dist = vec![INF; h * w];
    dist[start] = 0;
    let mut que = VecDeque::new();
    que.push_back(start);

    let mut teleport_used = [false; 26];

    while let Some(v) = que.pop_front() {
        let cur_dist = dist[v];
        let row = v / w;
        let col = v % w;

        for &(dr, dc) in &D4 {
            let next_row = row.wrapping_add(dr);
            let next_col = col.wrapping_add(dc);
            if next_row < h && next_col < w && graph[next_row][next_col] != '#' {
                let next_v = next_row * w + next_col;
                if dist[next_v] > cur_dist + 1 {
                    dist[next_v] = cur_dist + 1;
                    que.push_back(next_v);
                }
            }
        }

        let teleport_ch = graph[row][col];
        if teleport_ch.is_ascii_lowercase() {
            let letter = teleport_ch as usize - 'a' as usize;
            if !teleport_used[letter] {
                teleport_used[letter] = true;
                for &next_v in &teleports[letter] {
                    if dist[next_v] > cur_dist + 1 {
                        dist[next_v] = cur_dist + 1;
                        que.push_back(next_v);
                    }
                }
            }
        }
    }
    return dist;
}