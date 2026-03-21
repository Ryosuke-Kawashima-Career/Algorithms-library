use proconio::{input, marker::Chars};
const INF: usize = 1 << 60;
const N1: usize = 1usize.wrapping_neg();
// Check: 下右上左の順に方向に番号をそれぞれ 0,1,2,3
const D4: [(usize, usize); 4] = [(1, 0), (0, 1), (N1, 0), (0, N1)];
// ABC431E
// Q. 頂点倍加でBFSを行う
// A. Paraphrase the problem to descend to the problem of 0-1BFS
// Tips: 下右上左の順に方向に番号をそれぞれ 0,1,2,3 と名付ける => XOR(⊕)でマスを表現できる
// 01-BFSは「辺コスト1の最短路問題を[幅優先探索](http://d.hatena.ne.jp/keyword/%C9%FD%CD%A5%C0%E8%C3%B5%BA%F7) で解く方法の拡張」ではありますが、これが幅優先（Breadth-First）なのかと言われるとかなり違う気がしています。あくまで優先されているのは**暫定最短距離の短さ**だからです。
// [Wikipedia](http://d.hatena.ne.jp/keyword/Wikipedia)には「[最良優先探索（Best-First Search）](https://ja.wikipedia.org/wiki/%E6%9C%80%E8%89%AF%E5%84%AA%E5%85%88%E6%8E%A2%E7%B4%A2)」なる言葉が載っていて、こちらであれば完璧に当てはまります。偶然にもどちらも略語はBFSになってしまいますね。
fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {h: usize, w: usize, s: [Chars; h]}
        // dist[y][x][direction] | start = (0, -1), Initial Direction = 1
        let dist: Vec<Vec<Vec<usize>>> = bfs_expanded(0, N1, 1, &s);
        let ans = dist[h - 1][w - 1][1];
        println!("{}", ans);
    }
}

fn xor_answer(c: char) -> usize {
    match c {
        'A' => 0,
        'B' => 1,
        'C' => 3,
        _ => 2,
    }
}

fn bfs_expanded(y0: usize, x0: usize, dir0: usize, graph: &Vec<Vec<char>>) -> Vec<Vec<Vec<usize>>> {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    let mut dist: Vec<Vec<Vec<usize>>> = vec![vec![vec![INF; 4]; w]; h];
    let mut que = std::collections::VecDeque::new();
    // (cost, y, x, direction)
    que.push_back((0, y0, x0, dir0));

    while let Some((cost, y, x, dir)) = que.pop_front() {
        let (dy, dx) = D4[dir];
        let next_y: usize = y.wrapping_add(dy);
        let next_x: usize = x.wrapping_add(dx);
        if next_y < h && next_x < w {
            for next_dir in 0..4 {
                let xor_move: usize = dir ^ next_dir;
                if xor_move == xor_answer(graph[next_y][next_x]) {
                    // when the cost is zero: You should use "push_front" !!!!!
                    if dist[next_y][next_x][next_dir] > cost {
                        dist[next_y][next_x][next_dir] = cost;
                        que.push_front((cost, next_y, next_x, next_dir));
                    }
                } else if xor_move != 2 {
                    if dist[next_y][next_x][next_dir] > cost + 1 {
                        dist[next_y][next_x][next_dir] = cost + 1;
                        que.push_back((cost + 1, next_y, next_x, next_dir));
                    }
                }
            }
        }
    }

    return dist;
}
