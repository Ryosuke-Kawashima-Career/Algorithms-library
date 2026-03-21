use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
// abc322D
// 全探索
// grid問題をsetで計算する
fn main() {
    input!{p: [[Chars; 4]; 3]}
    let mut tetris: Vec<Vec<(i64, i64)>> = vec![vec![]; 3];
    for id in 0..3 {
        for i in 0..4 {
            for j in 0..4 {
                if p[id][i][j] == '#' {
                    tetris[id].push((i as i64, j as i64));
                }
            }
        }
    }
    let mut dy: Vec<i64> = vec![0; 3];
    let mut dx: Vec<i64> = vec![0; 3];

    if dfs(&mut tetris, &mut dy, &mut dx, 3) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn dfs(tetris: &mut Vec<Vec<(i64, i64)>>, dy: &mut Vec<i64>, dx: &mut Vec<i64>, cnt: usize) -> bool {
    let mut is_possible: bool = false;
    // corner case
    if cnt == 0 {
        // モジュール化する
        if is_in(tetris, dy, dx)
        && without_dup(tetris, dy, dx) 
        && is_filled(tetris, dy, dx) {
            is_possible = true;
        }
        return is_possible;
    }
    for _rot in 0..4 {
        move_left_up(&mut tetris[cnt-1]);
        for di in 0..4 {
            for dj in 0..4 {
                // 行きがけに変位を追加
                dy[cnt-1] = di;
                dx[cnt-1] = dj;
                // 盤上にテトリスの図形がすべて収まる場合が一つでもあるとき
                if dfs(tetris, dy, dx, cnt-1) {
                    is_possible = true;
                }
            }
        }
        rotate(&mut tetris[cnt-1]);
    }
    return is_possible;
}

fn move_left_up(tetris: &mut Vec<(i64, i64)>) {
    let n: usize = tetris.len();
    let mut min_y: i64 = 4;
    let mut min_x: i64 = 4;
    for &(y, x) in tetris.iter() {
        min_y = min_y.min(y);
        min_x = min_x.min(x);
    }
    for i in 0..n {
        let (y, x) = tetris[i];
        tetris[i] = (y - min_y, x - min_x);
    }
}

fn is_in(tetris: &Vec<Vec<(i64, i64)>>, dy: &Vec<i64>, dx: &Vec<i64>) -> bool {
    let mut in_grid: bool = true;
    for id in 0..3 {
        for &(y, x) in tetris[id].iter() {
            let moved_y: i64 = y + dy[id]; 
            let moved_x: i64 = x + dx[id];
            if moved_y < 0 || moved_y >= 4 
            || moved_x < 0 || moved_x >= 4 {
                in_grid = false;
            }
        }
    }
    return in_grid;
}

fn without_dup(tetris: &Vec<Vec<(i64, i64)>>, dy: &Vec<i64>, dx: &Vec<i64>) -> bool {
    let mut no_dup: bool = true;
    let mut grid = HashSet::new();
    for id in 0..3 {
        for &(y, x) in tetris[id].iter() {
            let moved_y: i64 = y + dy[id]; 
            let moved_x: i64 = x + dx[id];
            if grid.contains(&(moved_y, moved_x)) {
                no_dup = false;
            }
            grid.insert((moved_y, moved_x));
        }
    }
    return no_dup;
}

fn is_filled(tetris: &Vec<Vec<(i64, i64)>>, dy: &Vec<i64>, dx: &Vec<i64>) -> bool {
    let mut all_filled: bool = true;
    let mut grid = HashSet::new();
    for id in 0..3 {
        for &(y, x) in tetris[id].iter() {
            let moved_y: i64 = y + dy[id]; 
            let moved_x: i64 = x + dx[id];
            grid.insert((moved_y, moved_x));
        }
    }
    for i in 0..4 {
        for j in 0..4 {
            if !grid.contains(&(i, j)) {
                all_filled = false;
            }
        }
    }
    return all_filled;
}

// rotate (x, y) -> clock: (y, -x), anti clock(-y, x)
fn rotate(tetris: &mut Vec<(i64, i64)>) {
    let n: usize = tetris.len();
    for i in 0..n {
        let (y, x) = tetris[i];
        tetris[i] = (x, -y);
    }
}