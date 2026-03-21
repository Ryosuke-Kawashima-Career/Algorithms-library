use proconio::input;
use itertools::Itertools;
use std::collections::BTreeSet;
// abc345D: O(n!*2^n*HW)
// タイルを敷き詰める問題
// タイルのマスを辞書順に見る: (0, 0),..,(0, w-1),(1, 0),..,(h-1, w-1)
// 辞書順からマスをみて，使われていないマスにタイルを置く
fn main() {
    input!{n: usize, h: i64, w: i64, tiles: [(i64, i64); n]}
    // 座標をBTreeSetで管理することで左上のマスを高速に計算する
    let mut graph = BTreeSet::new();
    for i in 0..h {
        for j in 0..w {
            graph.insert((i, j));
        }
    }

    let mut is_ok = false;
    // タイルの置く順番をpermutationで決める
    for perm in (0..n).permutations(n) {
        if dfs(n, h, w, &tiles, &perm, &mut graph) {
            is_ok = true;
        }
    }

    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn dfs(phase: usize, h: i64, w: i64, tiles: &Vec<(i64, i64)>, 
    order: &Vec<usize>, graph: &mut BTreeSet<(i64, i64)>
) -> bool {
    let mut is_possible = false;
    // base case
    if phase == 0 {
        if graph.len() == 0 {
            return true;
        }
        return false;
    }

    let (height, width) = tiles[order[phase-1]];
    let (y0, x0) = detect_upper_left_corner(graph, h, w);
    // already full
    if y0 == h && x0 == w {
        return true;
    }
    // タイルを置かない
    if dfs(phase-1, h, w, tiles, order, graph) {
        is_possible = true;
    }
    if can_put((y0, x0), (height, width), graph) {
        erase_squares((y0, x0), (height, width), graph);
        if dfs(phase-1, h, w, tiles, order, graph) {
            is_possible = true;
        }
        // 帰りがけに復元する
        undo_erase_squares((y0, x0), (height, width), graph);
    }
    // 90度回転する
    if can_put((y0, x0), (width, height), graph) {
        erase_squares((y0, x0), (width, height), graph);
        if dfs(phase-1, h, w, tiles, order, graph) {
            is_possible = true;
        }
        // 帰りがけに復元する
        undo_erase_squares((y0, x0), (width, height), graph);
    }
    return is_possible;
}

fn detect_upper_left_corner(graph: &BTreeSet<(i64, i64)>, h: i64, w: i64) -> (i64, i64) {
    if let Some(&(y0, x0)) = graph.iter().next() {
        return (y0, x0);
    }
    return (h, w);
}

fn can_put((corner_y, corner_x): (i64, i64), (height, width): (i64, i64), 
    graph: &BTreeSet<(i64, i64)>
) -> bool {
    for dy in 0..height {
        for dx in 0..width {
            if !graph.contains(&(corner_y+dy, corner_x+dx)) {
                return false;
            }
        }
    }
    // if the graph has all squares
    return true;
}

fn erase_squares((corner_y, corner_x): (i64, i64), (height, width): (i64, i64), 
    graph: &mut BTreeSet<(i64, i64)>)
{
    for dy in 0..height {
        for dx in 0..width {
            graph.remove(&(corner_y+dy, corner_x+dx));
        }
    }
}

fn undo_erase_squares((corner_y, corner_x): (i64, i64), (height, width): (i64, i64), 
    graph: &mut BTreeSet<(i64, i64)>) 
{
    for dy in 0..height {
        for dx in 0..width {
            graph.insert((corner_y+dy, corner_x+dx));
        }
    }
}