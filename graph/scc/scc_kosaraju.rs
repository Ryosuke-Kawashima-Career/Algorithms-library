use proconio::{input, marker::Usize1};
// abc311C
// Functional Graph: N頂点N辺の有向グラフであって、各頂点から丁度一つの辺が出ているグラフ
// 有向閉路: 始点と終点が同じ循環するパス
// scc(互いに行き来できる頂点の集合): サイクルは強連結成分である
fn main() {
    input!{n: usize, a: [Usize1; n]}
    let mut scc = Scc::new(n);
    for v in 0..n {
        scc.add_edge(v, a[v]);
    }
    let groups = scc.clusters();
    for group in groups.iter() {
        if group.len() > 1 {
            print_cycle(group);
            return;
        }
    }
}

struct Scc {
    n: usize,
    graph: Vec<Vec<usize>>,
    graph_rev: Vec<Vec<usize>>,
}

impl Scc {
    fn new(n: usize) -> Self {
        Self {
            n,
            graph: vec![vec![]; n],
            graph_rev: vec![vec![]; n],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.graph[from].push(to);
        self.graph_rev[to].push(from);
    }

    pub fn topological_sort(&self) -> Vec<usize> {
        let mut seen = vec![false; self.n];
        let mut order: Vec<usize> = Vec::new();

        for v in 0..self.n {
            if !seen[v] {
                self.dfs(v, &mut seen, &mut order);
            }
        }
        order.reverse();
        return order;
    }

    fn dfs(&self, v: usize, seen: &mut Vec<bool>, order: &mut Vec<usize>) {
        seen[v] = true;
        for &next in self.graph[v].iter() {
            if !seen[next] {
                self.dfs(next, seen, order);
            }
        }
        // 帰りがけに記録する
        order.push(v);
    }

    // return [[node of group1; c1], [node of group2; c2],..]
    pub fn clusters(&self) -> Vec<Vec<usize>> {
        let order = self.topological_sort();
        let mut seen_rev = vec![false; self.n];
        let mut groups: Vec<Vec<usize>> = Vec::new();

        for i in 0..self.n {
            if !seen_rev[order[i]] {
                let mut order_rev: Vec<usize> = Vec::new();
                self.dfs_rev(order[i], &mut seen_rev, &mut order_rev);
                groups.push(order_rev);
            }
        }
        return groups;
    }

    fn dfs_rev(&self, v: usize, seen_rev: &mut Vec<bool>, order_rev: &mut Vec<usize>) {
        seen_rev[v] = true;
        for &next in self.graph_rev[v].iter() {
            if !seen_rev[next] {
                self.dfs_rev(next, seen_rev, order_rev);
            }
        }
        // 帰りがけに記録する
        order_rev.push(v);
    }
}

fn print_cycle(cycle: &Vec<usize>) {
    let k: usize = cycle.len();
    println!("{}", k);
    for i in 0..k {
        print!("{} ", cycle[i]+1);
    }
    println!("");
}