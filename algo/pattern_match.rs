use proconio::{input, marker::Chars};
use std::collections::HashSet;
// abc341C
// 座標をHashSetで管理する
fn main() {
    input!{h: usize, w: usize, n: usize, t: Chars, s: [Chars; h]}
    // patternは視点からの相対誤差
    let pattern: Vec<(i64, i64)> = create_pattern(&t);
    let coord: HashSet<(i64, i64)> = create_coord(&s);
    
    let mut ans: usize = 0;
    for i in 0..h {
        for j in 0..w {
            // (i, j)をpatternの始点とする
            if s[i][j] == '.' && is_same_pattern(i as i64, j as i64, &pattern, &coord) {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

// y0, x0をpatternの始点とする
fn is_same_pattern(y0: i64, x0: i64, 
    pattern: &Vec<(i64, i64)>, coord: &HashSet<(i64, i64)>
) -> bool {
    let mut is_same = true;
    for &(dy, dx) in pattern {
        let y: i64 = y0 + dy;
        let x: i64 = x0 + dx;
        if !coord.contains(&(y, x)) {
            is_same = false;
        }
    }
    return is_same;
}

fn create_pattern(operations: &Vec<char>) -> Vec<(i64, i64)> {
    let mut pattern: Vec<(i64, i64)> = Vec::new();
    let mut cur_y: i64 = 0;
    let mut cur_x: i64 = 0;
    pattern.push((cur_y, cur_x));
    let n: usize = operations.len();

    for i in 0..n {
        match operations[i] {
            'L' => {
                cur_x -= 1;
                pattern.push((cur_y, cur_x));
            },
            'R' => {
                cur_x += 1;
                pattern.push((cur_y, cur_x));
            },
            'U' => {
                cur_y -= 1;
                pattern.push((cur_y, cur_x));
            },
            'D' => {
                cur_y += 1;
                pattern.push((cur_y, cur_x));
            },
            _ => println!("Input Error!"),
        }
    }

    return pattern;
}

fn create_coord(graph: &Vec<Vec<char>>) -> HashSet<(i64, i64)> {
    let mut coord: HashSet<(i64, i64)> = HashSet::new();
    let h: usize = graph.len();
    let w: usize = graph[0].len();

    for i in 0..h {
        for j in 0..w {
            if graph[i][j] == '.' {
                coord.insert((i as i64, j as i64));
            }
        }
    }

    return coord;
}