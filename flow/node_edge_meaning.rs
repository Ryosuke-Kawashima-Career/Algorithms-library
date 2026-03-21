use proconio::input;
use proconio::marker::Chars;
// 鉄則B69
// エッジに意味を持たせる．
// 0: start -> 1..=n: employees -> n+1..=n+24: working hours -> n+25: end
fn main() {
    input!{n: usize, m: i64, c: [Chars; n]}
    let end: usize = n + 25;
    let mut ff = FordFulkerson::new(end + 1);
    for employee in 1..=n {
        // 労働時間は一日10時間まで
        ff.add_edge(0, employee, 10);
    }
    for employee in 1..=n {
        for j in 1..=24 {
            let time: usize = j + n;
            if c[employee-1][j-1] == '1' {
                ff.add_edge(employee, time, 1);
            }
        }
    }
    for j in 1..=24 {
        let time: usize = n + j;
        ff.add_edge(time, end, m);
    }

    let working_sum: i64 = ff.max_flow(0, end);
    if working_sum == 24 * m {
        println!("Yes");
    } else {
        println!("No");
    }
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