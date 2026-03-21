use proconio::input;
// abc330e
fn main() {
    input!{n: usize, q: usize, mut a: [i64; n]}
    let mut set = RangeSet::new();
    let mut count: Vec<usize> = vec![0; n+1];

    for i in 0..n {
        set.insert(a[i]);
        if a[i] <= n as i64 {
            count[a[i] as usize] += 1;
        }
    }
    
    for _ in 0..q {
        input!{index: usize, x: i64}
        let index = index - 1;
        if a[index] <= n as i64 {
            count[a[index] as usize] -= 1;
        }
        if a[index] <= n as i64 && count[a[index]as usize] == 0 {
            set.remove(a[index]);
        }
        a[index] = x;
        set.insert(x);
        if a[index] <= n as i64 {
            count[a[index] as usize] += 1;
        }
        let min_value: i64 = set.mex(0);
        println!("{}", min_value);
    }
}

// 集合とそのmexを管理する
// {[-INF, -INF], [0, 3], [5, 5], [INF, INF]}: 閉区間
pub struct RangeSet {
    pub ranges: std::collections::BTreeSet<(i64, i64)>,
}
impl std::fmt::Debug for RangeSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.ranges)
    }
}
impl RangeSet {
    /// RangeSetを初期化する
    pub fn new() -> Self {
        let ranges = [(i64::MIN, i64::MIN), (i64::MAX, i64::MAX)]
            .into_iter()
            .collect();
        Self { ranges }
    }

    pub fn contains(&self, x: i64) -> bool {
        match self.ranges.range(..(x + 1, i64::MAX)).next_back() {
            Some(&(l, r)) => l <= x && x <= r,
            None => false,
        }
    }

    //[l, r] is covered?
    pub fn covered(&self, l: i64, r: i64) -> bool {
        if l > r {
            return false;
        }
        // prev of upper_bound
        match self.ranges.range(..(l + 1, i64::MAX)).next_back() {
            Some(&(lrange, rrange)) => lrange <= l && r <= rrange,
            None => false,
        }
    }

    /// 集合に要素`x`を追加する
    /// ### 戻り値
    /// - `true`: `x`が追加された場合
    /// - `false`: `x`がすでに存在していた場合
    pub fn insert(&mut self, x: i64) -> bool {
        let &(ll, l) = self.ranges.range(..(x + 1, x + 1)).next_back().unwrap();
        let &(r, rr) = self.ranges.range((x + 1, x + 1)..).next().unwrap();
        if x <= l {
            return false;
        }
        match (l == x - 1, x + 1 == r) {
            (false, false) => {
                self.ranges.insert((x, x));
            }
            (false, true) => {
                self.ranges.remove(&(r, rr));
                self.ranges.insert((x, rr));
            }
            (true, false) => {
                self.ranges.remove(&(ll, l));
                self.ranges.insert((ll, x));
            }
            (true, true) => {
                self.ranges.remove(&(ll, l));
                self.ranges.remove(&(r, rr));
                self.ranges.insert((ll, rr));
            }
        }
        true
    }
    /// 集合から要素`x`を削除する
    /// ### 戻り値
    /// - `true`: `x`が削除された場合
    /// - `false`: `x`がすでに存在していなかった場合
    pub fn remove(&mut self, x: i64) -> bool {
        let &(ll, l) = self.ranges.range(..(x + 1, x + 1)).next_back().unwrap();
        if l < x {
            return false;
        }
        self.ranges.remove(&(ll, l));
        match (ll == x, x == l) {
            (false, false) => {
                self.ranges.insert((ll, x - 1));
                self.ranges.insert((x + 1, l));
            }
            (false, true) => {
                self.ranges.insert((ll, x - 1));
            }
            (true, false) => {
                self.ranges.insert((x + 1, l));
            }
            (true, true) => (),
        }
        true
    }
    /// **集合に含まれない**`x`以上で最小の整数を調べる
    pub fn mex(&self, x: i64) -> i64 {
        let &(ll, l) = self.ranges.range(..(x + 1, x + 1)).next_back().unwrap();
        if ll <= x && x <= l {
            l + 1
        } else {
            x
        }
    }
}