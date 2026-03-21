use proconio::input;
use proconio::marker::Chars;
// abc322D
// 全探索
fn main() {
    input!{p1: [Chars; 4], p2: [Chars; 4], p3: [Chars; 4]}
    let mut tetris: Vec<Vec<Vec<char>>> = vec![p1, p2, p3];
    let mut is_possible: bool = false;
    // テトリスを回転
    for _rot0 in 0..4 {
        for _rot1 in 0..4 {
            for _rot2 in 0..4 {
                // テトリス一つ一つをそれぞれ動かす!!!
                for dy0 in -4..=4 {
                    for dx0 in -4..=4 {
                        for dy1 in -4..=4 {
                            for dx1 in -4..=4 {
                                for dy2 in -4..=4 {
                                    for dx2 in -4..=4 {
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
                let tetris2: Vec<Vec<char>> = rotate_right(&tetris[2]);
                tetris[2] = tetris2;
            }
            let tetris1: Vec<Vec<char>> = rotate_right(&tetris[1]);
            tetris[1] = tetris1;
        }
        let tetris0: Vec<Vec<char>> = rotate_right(&tetris[0]);
        tetris[0] = tetris0;
    }

    if is_possible {
        println!("Yes");
    } else {
        println!("No");
    }
}
// 関数を意味ごとに分けて作成する
fn is_in(tetris: &Vec<Vec<Vec<char>>>, dy: &Vec<i64>, dx: &Vec<i64>) -> bool {
    let mut in_grid: bool = true;
    for id in 0..3 {
        for i in 0..4 {
            for j in 0..4 {
                if tetris[id][i][j] == '#' {
                    let moved_i: i64 = i as i64 + dy[id]; 
                    let moved_j: i64 = j as i64 + dx[id];
                    if moved_i < 0 || moved_i >= 4 
                    || moved_j < 0 || moved_j >= 4 {
                        in_grid = false;
                    }
                }
            }
        }
    }
    return in_grid;
}

fn without_dup(tetris: &Vec<Vec<Vec<char>>>, dy: &Vec<i64>, dx: &Vec<i64>) -> bool {
    let mut no_dup: bool = true;
    let mut used: Vec<Vec<bool>> = vec![vec![false; 4]; 4];
    for id in 0..3 {
        for i in 0..4 {
            for j in 0..4 {
                if tetris[id][i][j] == '#' {
                    let moved_i: i64 = i as i64 + dy[id]; 
                    let moved_j: i64 = j as i64 + dx[id];
                    if 0 <= moved_i && moved_i < 4 
                    && 0 <= moved_j && moved_j < 4 {
                        if used[moved_i as usize][moved_j as usize] {
                            no_dup = false;
                        }
                        used[moved_i as usize][moved_j as usize] = true;
                    }
                }
            }
        }
    }
    return no_dup;
}

fn is_filled(tetris: &Vec<Vec<Vec<char>>>, dy: &Vec<i64>, dx: &Vec<i64>) -> bool {
    let mut all_filled: bool = true;
    let mut used: Vec<Vec<bool>> = vec![vec![false; 4]; 4];
    for id in 0..3 {
        for i in 0..4 {
            for j in 0..4 {
                if tetris[id][i][j] == '#' {
                    let moved_i: i64 = i as i64 + dy[id]; 
                    let moved_j: i64 = j as i64 + dx[id];
                    if 0 <= moved_i && moved_i < 4 
                    && 0 <= moved_j && moved_j < 4 {
                        used[moved_i as usize][moved_j as usize] = true;
                    }
                }
            }
        }
    }
    for i in 0..4 {
        for j in 0..4 {
            if !used[i][j] {
                all_filled = false;
            }
        }
    }
    return all_filled;
}

// rotate left <-> rotate right: res[0][0]のマスがpのどのマスから来たのか着目
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