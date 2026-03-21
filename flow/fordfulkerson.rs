use proconio::input;

// 最大流　= 最小カット容量
// エッジのコストは正
// 蟻本(p190)
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

// ノードやエッジの番号に意味を持たせる
fn main() {
    // n: node, g: number of destinations, e: number of edges
    input!{n: usize, g: usize, e: usize, targets: [usize; g], edges: [(usize, usize); e]}
    // すべての目的地を一つのノードにつなぐ。
    let v: usize = n + 1;
    let mut ff = FordFulkerson::new(v);

    for &p in targets.iter() {
        // 複数の目的地を一つの終点につなぐ。
        ff.add_edge(p, n, 1);
    }

    for &(from, to) in edges.iter() {
        ff.add_edge(from, to, 1);
        ff.add_edge(to, from, 1);
    }

    // 始点から終点の最大流つまり最小カット容量を計算する。
    let ans: i64 = ff.max_flow(0, n);
    println!("{}", ans);
}