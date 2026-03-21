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

// Tarjan's SCC
// 1. tree edge: dfsに使われるedge
// 2. forward edge: v -> u(child)
// 3. back edge: v -> u(parent)
// 4. cross edge: v -> u(子孫の関係でない)
// トポロジカル順序の逆順にscc-idが付与されていく
// scc_rootでないとき
// 1. back edgeが出ている
// 2. 帰りがけ時点で未確定の頂点へのcross edge
// 3. 帰りがけ時点で未確定の子孫のうち、order[v]より小さい order の未確定の頂点に到達可能なものが存在する
// low-linkの定義: DFSでの頂点の帰りがけの時点で以下の２種類の方法によってたどり着けるscc-idが未確定である頂点のorderの最小値
// 1. tree edge を任意の回数使用する
// 2. back edge もしくは cross edge を高々１回使用する
// vがscc-rootのとき，order[v] = low-link[v]

struct Scc {
    n: usize,
    graph: Vec<Vec<usize>>,
}

impl Scc {
    fn new(n: usize) -> Self {
        Self {
            n,
            graph: vec![vec![]; n],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.graph[from].push(to);
    }

    fn scc_ids(&self) -> (usize, Vec<usize>, Vec<usize>) {
        let mut group_num: usize = 0;
        let mut cur_order: usize = 0;
        // 探索済みかつidが未確定の頂点を管理するstack
        let mut visited: Vec<usize> = Vec::new();
        let mut topological_order: Vec<usize> = Vec::new();
        let mut low_links: Vec<usize> = vec![0; self.n];
        let mut orders: Vec<usize> = vec![self.n; self.n];
        let mut ids: Vec<usize> = vec![0; self.n];

        for v in 0..self.n {
            // if not visited
            if orders[v] == self.n {
                self.dfs(
                    v, &mut group_num, &mut cur_order, 
                    &mut visited, &mut low_links, &mut orders, &mut ids,
                    &mut topological_order
                );
            }
        }
        // scc-idはトポロジカル順序の逆順に付与されているので反転する
        for v in 0..self.n {
            ids[v] = group_num - 1 - ids[v];
        }
        topological_order.reverse();
        return (group_num, ids, topological_order);
    }

    fn dfs(&self, v: usize, group_num: &mut usize, cur_order: &mut usize, 
        visited: &mut Vec<usize>, low_links: &mut Vec<usize>, 
        orders: &mut Vec<usize>, ids: &mut Vec<usize>, topological_order: &mut Vec<usize>) 
    {
        // 行きがけ
        // 1. orderを付与 2. lowlinkをorderで初期化 3. stackに頂点を追加
        low_links[v] = *cur_order;
        orders[v] = *cur_order;
        *cur_order += 1;
        visited.push(v);

        for &next in self.graph[v].iter() {
            // 未探索の時(tree edge)
            if orders[next] == self.n {
                self.dfs(
                    next, group_num, cur_order, 
                    visited, low_links, orders, ids, topological_order
                );
                // tree edgeは何回でも使えるので vのlowlink <= nextのlowlink
                let low_link_next = low_links[next];
                low_links[v] = low_links[v].min(low_link_next);
            } else {
                // tree edgeでない辺は1回しか使えないので行き先のorderでlowlinkを更新
                let order_next = orders[next];
                low_links[v] = low_links[v].min(order_next);
            }
        }

        // 帰りがけ
        // vがscc_rootのときstackのv以降にscc-idを付与
        // orderをn+1(無限大と同等)に変更し,これ以降lowlinkの計算に使われないようにする
        if low_links[v] == orders[v] {
            while let Some(u) = visited.pop() {
                orders[u] = self.n + 1;
                ids[u] = *group_num;
                if u == v {
                    break;
                }
            }
            *group_num += 1;
        }
        topological_order.push(v);
    }

    // return [[node of group1; c1], [node of group2; c2],..]
    pub fn clusters(&self) -> Vec<Vec<usize>> {
        let (group_num, ids, topological_order) = self.scc_ids();
        let mut groups: Vec<Vec<usize>> = vec![vec![]; group_num];

        for v in 0..self.n {
            groups[ids[topological_order[v]]].push(topological_order[v]);
        }
        return groups;
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