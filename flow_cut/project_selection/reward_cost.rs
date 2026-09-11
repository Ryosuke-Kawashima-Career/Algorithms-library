use proconio::input;
const INF: i64 = 1 << 60;
// 鉄則B69
// 最大流問題: マッチングが関係するとき
// マッチングが強制の時，エッジの大きさをinfにする
// start ->(if cost positive) node ->(if cost negative) end
// start -> node: 特急駅にしないときのコスト
// node -> end: 特急駅にするときのコスト
fn main() {
    input!{n: usize, m: usize, p: [i64; n], ab: [(usize, usize); m]}
    let mut ff = FordFulkerson::new(n + 2);
    for i in 0..m {
        ff.add_edge(ab[i].0, ab[i].1, INF);
    }
    let mut offset: i64 = 0;
    for v in 1..=n {
        if p[v-1] >= 0 {
            ff.add_edge(0, v, p[v-1]);
            offset += p[v-1];
        } else {
            ff.add_edge(v, n+1, p[v-1].abs());
        }
    }

    let max_reward: i64 = offset - ff.max_flow(0, n+1);
    println!("{}", max_reward);
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Edge {
    to: usize,
    cap: i64,
    rev: usize
}

struct FordFulkerson {
    g: Vec<Vec<Edge>>,
    used: Vec<bool>
}

impl FordFulkerson {
    fn new(n: usize) -> FordFulkerson {
        let g: Vec<Vec<_>> = vec![vec![]; n];
        let used: Vec<bool> = vec![false; n];
        FordFulkerson {g, used}
    }

    fn add_edge(&mut self, from:usize, to:usize, cap:i64) {
        let to_edge = Edge {
            to, cap, rev: self.g[to].len()
        };
        self.g[from].push(to_edge);
        let from_edge = Edge {
            to: from, cap: 0, rev: self.g[from].len() - 1
        };
        self.g[to].push(from_edge);
    }

    fn dfs(&mut self, v:usize, t:usize, f:i64) -> i64 {
        if v == t {
            return f
        }

        self.used[v] = true;
        for i in 0..self.g[v].len() {
            let edge = self.g[v][i];
            if !self.used[edge.to] && 0 < edge.cap {
            let d = self.dfs(edge.to, t, std::cmp::min(f, edge.cap));
                if 0 < d {
                    self.g[v][i].cap -= d;
                    self.g[edge.to][edge.rev].cap += d;
                    return d
                }
            }
        }
        0
    }

    fn max_flow(&mut self, s:usize, t:usize) -> i64 {
        let mut flow = 0;
        loop {
            self.used = vec![false; self.used.len()];
            let f = self.dfs(s, t, std::i64::MAX);
            if f == 0 {
                return flow
            }
            flow += f;
        }
    }
}