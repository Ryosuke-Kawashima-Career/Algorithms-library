use proconio::{input, marker::Chars};
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [
    (N1, 0),
    (0, N1), (0, 1),
    (1, 0)
];
// 典型72
// Q. どれだけ空のマスを同じ場所を通らずに通過できるか？
fn main() {
    input!{h: usize, w: usize, c: [Chars; h]}
    let mut seen: Vec<Vec<bool>> = vec![vec![false; w]; h];
    let mut moves: usize = 0;
    for h0 in 0..h {
        for w0 in 0..w {
            if c[h0][w0] == '.' {
                let (okay, moves_cur) = dfs(h0, w0, h0, w0, &c, &mut seen);
                if okay {
                    moves = moves.max(moves_cur);
                }
            }
        }
    }

    if moves >= 3 {
        println!("{}", moves);
    } else {
        println!("-1");
    }
}
// return the result of okay and moves
fn dfs(h0: usize, w0: usize, h_cur: usize, w_cur: usize, 
    graph: &Vec<Vec<char>>, seen: &mut Vec<Vec<bool>>
) -> (bool, usize)
{
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    if seen[h_cur][w_cur] && h_cur == h0 && w_cur == w0 {
        // (okay?, the number of moves)
        return (true, 0);
    }
    // if already seen
    if seen[h_cur][w_cur] {
        return (false, 0);
    }
    let mut res: usize = 0;
    seen[h_cur][w_cur] = true;

    for &(dy, dx) in D4.iter() {
        let next_y: usize = h_cur.wrapping_add(dy);
        let next_x: usize = w_cur.wrapping_add(dx);
        if next_y < h && next_x < w && graph[next_y][next_x] == '.' {
            let (okay, moves) = dfs(h0, w0, next_y, next_x, graph, seen);
            if okay {
                res = res.max(moves + 1);
            }
        }
    }
    
    seen[h_cur][w_cur] = false;
    if res == 0 {
        return (false, 0);
    }
    return (true, res);
}