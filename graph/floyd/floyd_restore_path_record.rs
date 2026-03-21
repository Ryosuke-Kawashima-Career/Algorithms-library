const INF: i64 = 1 << 60;
pub struct Floyd {
    n: usize,
    // dp[i][j]: iからjへの最短路のコストを格納
    dp: Vec<Vec<i64>>,
    // prev[i][j]: iからjへの最短路でのjの1つ前の点を格納
    prev: Vec<Vec<usize>>,
    // next[i][j]: iからjへの最短路でのjの1つ後の点を格納
    next: Vec<Vec<usize>>,
}
impl Floyd {
    fn new(n: usize) -> Self {
        let mut dp: Vec<Vec<i64>> = vec![vec![INF; n]; n];
        let mut prev: Vec<Vec<usize>> = vec![vec![n; n]; n];
        let mut next: Vec<Vec<usize>> = vec![vec![n; n]; n];
        for i in 0..n {
            dp[i][i] = 0;
        }
        for i in 0..n {
            for j in 0..n {
                prev[i][j] = i;
                next[i][j] = j;
            }
        }
        Self {
            n,
            dp,
            prev,
            next,
        }
    }
    // 隣接行列を返す: 有向グラフか無向グラフかに気を付ける!!!(edges: 0indexed)
    pub fn set_edges(&mut self, edges: &Vec<(usize, usize, i64)>) {
        // customize!!!
        for &(from, to, cost) in edges.iter() {
            self.dp[from][to] = cost;
            self.dp[to][from] = cost;
        }
    }

    // 計算量: O(|V|^3)
    // k: 中継点 -> u -> v
    pub fn floyd(&mut self) {
        for k in 0..self.n {
            for i in 0..self.n {
                for j in 0..self.n {
                    if self.dp[i][k] != INF && self.dp[k][j] != INF
                    && self.dp[i][j] > self.dp[i][k] + self.dp[k][j] 
                    {
                        self.dp[i][j] = self.dp[i][k] + self.dp[k][j];
                        // i -> k -> j
                        self.prev[i][j] = self.prev[k][j];
                        self.next[i][j] = self.next[i][k];
                    }
                }
            }
        }
    }

    // 計算量: O(|V|)
    // self.floyd()の後につかう
    // goalから遡る
    pub fn get_path_prev(&self, start: usize, goal: usize) -> Vec<usize> {
        let mut path: Vec<usize> = Vec::new();
        path.push(goal);
        let mut cur = goal;

        while cur != start {
            // start -> prev -> cur
            cur = self.prev[start][cur];
            path.push(cur);
        }

        path.reverse();
        return path;
    }
    // 計算量: O(|V|)
    // self.floyd()の後につかう
    // startから進む
    pub fn get_path_next(&self, start: usize, goal: usize) -> Vec<usize> {
        let mut path: Vec<usize> = Vec::new();
        path.push(start);
        let mut cur = start;

        while cur != goal {
            cur = self.next[cur][goal];
            path.push(cur);
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