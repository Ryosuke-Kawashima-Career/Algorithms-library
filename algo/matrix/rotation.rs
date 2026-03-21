use proconio::input;
use proconio::marker::Chars;
// abc322D
// general rotation
// MxN -> NxM
fn rotate_right(p: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let m: usize = p.len();
    let n: usize = p[0].len();
    let mut res: Vec<Vec<char>> = vec![vec!['o'; m]; n];
    for i in 0..n {
        for j in 0..m {
            res[i][j] = p[(m-1)-j][i];
        }
    }
    return res;
}
fn rotate_left(p: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let m: usize = p.len();
    let n: usize = p[0].len();
    let mut res: Vec<Vec<char>> = vec![vec!['o'; m]; n];
    for i in 0..n {
        for j in 0..m {
            res[i][j] = p[j][(n-1)-i];
        }
    }
    return res;
}

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
    let mut is_possible: bool = false;
    // テトリスを回転
    for _rot0 in 0..4 {
        move_left_up(&mut tetris[0]);
        for _rot1 in 0..4 {
            move_left_up(&mut tetris[1]);
            for _rot2 in 0..4 {
                move_left_up(&mut tetris[2]);
                // テトリス一つ一つをそれぞれ動かす!!!
                for dy0 in 0..4 {
                    for dx0 in 0..4 {
                        for dy1 in 0..4 {
                            for dx1 in 0..4 {
                                for dy2 in 0..4 {
                                    for dx2 in 0..4 {
                                        let dy = vec![dy0, dy1, dy2];
                                        let dx = vec![dx0, dx1, dx2];
                                        // モジュール化する
                                        if is_in(&tetris, &dy, &dx)
                                        && without_dup(&tetris, &dy, &dx) 
                                        && is_filled(&tetris, &dy, &dx) {
                                            is_possible = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                rotate(&mut tetris[2]);
            }
            rotate(&mut tetris[1]);
        }
        rotate(&mut tetris[0]);
    }

    if is_possible {
        println!("Yes");
    } else {
        println!("No");
    }
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