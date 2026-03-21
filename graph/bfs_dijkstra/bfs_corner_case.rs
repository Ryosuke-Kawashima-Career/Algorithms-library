use proconio::{input, marker::Chars};
use std::collections::HashMap;
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [
    (N1, 0),
    (0, N1), (0, 1),
    (1, 0)
];
// abc308D
// 境界条件を設定するとなぜかめっちゃ速くなる?
// s -> n -> u -> k -> e　の順にマスを進む
fn main() {
    input!{h: usize, w: usize, s: [Chars; h]}
    let pattern = "snuke".chars().collect::<Vec<char>>();
    // 文字列のindex: ex. s: 0, n: 1,..
    let mut map = HashMap::new();
    for i in 0..pattern.len() {
        map.insert(pattern[i], i);
    }
    let can_be_seen: Vec<Vec<bool>> = bfs((0, 0), &s, &map);
    if can_be_seen[h-1][w-1] {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn bfs((h0, w0): (usize, usize), graph: &Vec<Vec<char>>, indexes: &HashMap<char, usize>) -> Vec<Vec<bool>> {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    let n: usize = indexes.len();
    let mut seen: Vec<Vec<bool>> = vec![vec![false; w]; h];
    // corner case
    if !indexes.contains_key(&graph[h0][w0]) {
        return seen;
    }
    seen[h0][w0] = true;
    let mut que = std::collections::VecDeque::new();
    que.push_back((h0, w0));

    while let Some((y, x)) = que.pop_front() {
        for &(dy, dx) in D4.iter() {
            let next_y: usize = y + dy;
            let next_x: usize = x + dx;
            if next_y < h && next_x < w && indexes.contains_key(&graph[next_y][next_x]) {
                let index = indexes[&graph[y][x]];
                let next_index = indexes[&graph[next_y][next_x]];
                // speed up by checking the next not seen!!!
                if !seen[next_y][next_x] && (index + 1) % n == next_index {
                    seen[next_y][next_x] = true;
                    que.push_back((next_y, next_x));
                }
            }
        }
    }

    return seen;
}