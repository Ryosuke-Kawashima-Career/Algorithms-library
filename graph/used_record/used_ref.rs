use proconio::input;
use proconio::marker::Chars;
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [
    (N1, 0),
    (0, N1), (0, 1),
    (1, 0)
];
// 典型72
// usedを参照する方が配列をコピーしないので速い
fn main() {
    input!{h: usize, w: usize, c: [Chars; h]}
    let mut k: usize = 0;

    let mut used: Vec<Vec<bool>> = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '.' {
                dfs(i, j, i, j, 0, &c, &mut used, &mut k);
            }
        }
    }

    if k < 3 {
        println!("-1");
    } else {
        println!("{}", k);
    }
}
// dfsが目的地にたどり着かないかもしれないので値を返さない。
fn dfs(y0: usize, x0: usize, y: usize, x: usize, cur_size: usize,
    graph: &Vec<Vec<char>>, used: &mut Vec<Vec<bool>>, size: &mut usize)
{
    if y == y0 && x == x0 && used[y][x] {
        *size = (*size).max(cur_size);
        return;
    }
    if used[y][x] {
        return;
    }
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    used[y][x] = true;
    for &(dy, dx) in D4.iter() {
        let next_y: usize = y + dy;
        let next_x: usize = x + dx;
        if next_y < h && next_x < w 
        && graph[next_y][next_x] == '.' {
            dfs(y0, x0, next_y, next_x, cur_size+1, 
                graph, used, size
            );
        }
    }

    // 帰りがけにusedを復元する
    used[y][x] = false;
}