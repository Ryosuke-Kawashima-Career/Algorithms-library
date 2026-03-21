const INF: i64 = 1 << 60;
pub struct Floyd {
    n: usize,
    dp: Vec<Vec<i64>>,
    original: Vec<Vec<i64>>,
}
impl Floyd {
    fn new(n: usize) -> Self {
        let mut dp: Vec<Vec<i64>> = vec![vec![INF; n]; n];
        for i in 0..n {
            dp[i][i] = 0;
        }
        let original = dp.clone();
        Self {
            n,
            dp,
            original,
        }
    }
    // 隣接行列を返す: 有向グラフか無向グラフかに気を付ける!!!(edges: 0indexed)
    pub fn set_edges(&mut self, edges: &Vec<(usize, usize, i64)>) {
        // customize!!!
        for &(from, to, cost) in edges.iter() {
            self.dp[from][to] = cost;
            self.dp[to][from] = cost;
            self.original[from][to] = cost;
            self.original[to][from] = cost;
        }
    }

    // 計算量: O(|V|^3)
    // k: 中継点 -> u -> v
    pub fn floyd(&mut self) {
        for k in 0..self.n {
            for i in 0..self.n {
                for j in 0..self.n {
                    if self.dp[i][k] != INF && self.dp[k][j] != INF {
                        self.dp[i][j] = self.dp[i][j].min(self.dp[i][k] + self.dp[k][j]);
                    }
                }
            }
        }
    }

    // 計算量: O(|V|^2)
    // self.floyd()の後につかう
    pub fn get_path(&self, start: usize, goal: usize) -> Vec<usize> {
        let mut path: Vec<usize> = Vec::new();
        path.push(start);
        let mut cur = start;

        while cur != goal {
            for next in 0..self.n {
                // nextがcur-goalの中継点か?
                if cur != next && self.original[cur][next] != INF && self.dp[next][goal] != INF
                && self.dp[cur][goal] == self.original[cur][next] + self.dp[next][goal]
                {
                    path.push(next);
                    cur = next;
                    break;
                }
            }
        }

        return path;
    }

    pub fn has_negative_loop(&self) -> bool {
        let mut exist_negative_loop = false;
        for i in 0..self.n {
            if self.dp[i][i] < 0 {
                exist_negative_loop = true;
            }
        }
        return exist_negative_loop;
    }
}