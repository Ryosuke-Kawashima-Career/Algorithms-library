use proconio::input;
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [
    (N1, 0),
    (0, N1), (0, 1),
    (1, 0)
];
// JOI8予選D 薄氷渡り
fn main() {
    // 0: 薄氷なし 1: あり
    input!{m: usize, n: usize, graph: [[usize; m]; n]}
    let mut max_size: usize = 0;
    // dfsの帰りがけで復元する
    let mut used: Vec<Vec<bool>> = vec![vec![false; m]; n];
    for i in 0..n {
        for j in 0..m {
            if graph[i][j] == 1 {
                let size: usize = dfs(i, j, 0, &graph, &mut used);
                max_size = max_size.max(size);
            }
        }
    }
    println!("{}", max_size);
}

// 部分木の深さを求めるアルゴリズムに類似
fn dfs(y0: usize, x0: usize, cur_size: usize,
    graph: &Vec<Vec<usize>>, used: &mut Vec<Vec<bool>>
) -> usize {
    used[y0][x0] = true;
    let mut size: usize = cur_size + 1;
    let h: usize = used.len();
    let w: usize = used[0].len();
    for &(dy, dx) in D4.iter() {
        let next_y: usize = y0 + dy;
        let next_x: usize = x0 + dx;
        if next_y < h && next_x < w
        && graph[next_y][next_x] == 1 && !used[next_y][next_x] {
            let next_size = dfs(next_y, next_x, cur_size, graph, used);
            // 帰りがけに計算する
            // 答えの候補は子供の大きさ + 1(親の大きさ)
            size = size.max(next_size + 1);
        }
    }

    used[y0][x0] = false;
    return size;
}