use proconio::{input, marker::Chars};
use std::collections::VecDeque;
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [
    (N1, 0),
    (0, N1), (0, 1),
    (1, 0)
];
// abc425D
// Q. if the number of adjacent black cells is one, the cell turns into black.
// A. maximum iteration is h * w (upper bound)
fn main() {
    input!{h: usize, w: usize, mut s: [Chars; h]}
    let mut is_checked: Vec<Vec<bool>> = vec![vec![false; w]; h];
    let mut adjacent_black: Vec<Vec<usize>> = vec![vec![0; w]; h];
    let mut que = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                is_checked[i][j] = true;
                que.push_back((i, j));
            }
        }
    }
    
    for _phase in 0..h*w {
        if !has_updated(&mut s, &mut is_checked, &mut adjacent_black, &mut que) {
            break;
        }
    }

    let ans = count_black(&s);
    println!("{}", ans);
}

fn has_updated(graph: &mut Vec<Vec<char>>, is_checked: &mut Vec<Vec<bool>>, adjacent_black: &mut Vec<Vec<usize>>, que: &mut VecDeque<(usize, usize)>) 
-> bool 
{
    let h: usize = graph.len();
    let w: usize = graph[0].len();

    let mut candidates: Vec<(usize, usize)> = Vec::new();
    while let Some((y, x)) = que.pop_front() {
        for &(dy, dx) in D4.iter() {
            let next_y: usize = y.wrapping_add(dy);
            let next_x: usize = x.wrapping_add(dx);
            if next_y < h && next_x < w {
                if !is_checked[next_y][next_x] && graph[next_y][next_x] == '.' {
                    adjacent_black[next_y][next_x] += 1;
                    if adjacent_black[next_y][next_x] > 1 {
                        is_checked[next_y][next_x] = true;
                    } else {
                        candidates.push((next_y, next_x));
                    }
                }
            }
        }
    }

    let mut any_update = false;
    for &(y_candidate, x_candidate) in candidates.iter() {
        if adjacent_black[y_candidate][x_candidate] == 1 
        && graph[y_candidate][x_candidate] == '.' 
        && !is_checked[y_candidate][x_candidate] 
        {
            graph[y_candidate][x_candidate] = '#';
            que.push_back((y_candidate, x_candidate));
            any_update = true;
            is_checked[y_candidate][x_candidate] = true;
        }
    }
    return any_update;
}

fn count_black(graph: &Vec<Vec<char>>) -> usize {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    let mut cnt: usize = 0;
    for i in 0..h {
        for j in 0..w {
            if graph[i][j] == '#' {
                cnt += 1;
            }
        }
    }
    return cnt;
}