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

// set: {[0, 4), [5, 6)} -> Mex = 4: 右が開区間
struct RangeSet {
    set: std::collections::BTreeSet<(i64, i64)>,
}

impl RangeSet {
    pub fn new() -> Self {
        Self {
            set: std::collections::BTreeSet::new(),
        }
    }
}

impl std::fmt::Debug for RangeSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.set)
    }
}

impl RangeSet {
    pub fn contains(&self, x: i64) -> bool {
        match self.set.range(..(x + 1, i64::MAX)).next_back() {
            Some(&(l, r)) => l <= x && x < r,
            None => false,
        }
    }

    //[l, r) is covered?
    #[allow(dead_code)]
    pub fn covered(&self, l: i64, r: i64) -> bool {
        if l >= r {
            return false;
        }
        // prev of upper_bound
        match self.set.range(..(l + 1, i64::MAX)).next_back() {
            Some(&(lrange, rrange)) => lrange <= l && r <= rrange,
            None => false,
        }
    }

    // Remove `x` from range if exist. Return true if x exist. Otherwise, return false.
    pub fn remove(&mut self, x: i64) -> bool {
        let range = self.set.range(..(x + 1, x + 1)).next_back();
        match range {
            Some(&(l, r)) => {
                if l == x {
                    self.set.remove(&(l, r));
                    if l + 1 != r {
                        self.set.insert((l + 1, r));
                    }
                } else if r == x + 1 {
                    self.set.remove(&(l, r));
                    if l != r - 1 {
                        self.set.insert((l, r - 1));
                    }
                } else {
                    self.set.remove(&(l, r));
                    if l != x {
                        self.set.insert((l, x));
                    }
                    if x + 1 != r {
                        self.set.insert((x + 1, r));
                    }
                }
                true
            }
            None => false,
        }
    }

    // Try to insert x. If there already exist range which contain x, return false.
    // Otherwise, insert x then return true.
    pub fn insert(&mut self, x: i64) -> bool {
        if self.contains(x) {
            return false;
        }

        let lrange = self.set.range(..(x + 1, x + 1)).next_back();
        let rrange = self.set.range((x + 1, x + 1)..).next();
        match (lrange, rrange) {
            (Some(&(l1, r1)), Some(&(l2, r2))) => {
                if r1 == x {
                    if l2 == x + 1 {
                        // merge [l1, x) and [x + 1, r2) by adding [x, x + 1)
                        self.set.remove(&(l1, r1));
                        self.set.remove(&(l2, r2));
                        self.set.insert((l1, r2));
                    } else {
                        // extend [l1, x) to [l1, x + 1)
                        self.set.remove(&(l1, r1));
                        self.set.insert((l1, x + 1));
                    }
                } else {
                    if l2 == x + 1 {
                        // extend [x + 1, r2) to [x, r2)
                        self.set.remove(&(l2, r2));
                        self.set.insert((x, r2));
                    } else {
                        // Just add [x, x + 1)
                        self.set.insert((x, x + 1));
                    }
                }
            }
            (Some(&(l, r)), None) => {
                if r == x {
                    self.set.remove(&(l, r));
                    self.set.insert((l, x + 1));
                } else {
                    self.set.insert((x, x + 1));
                }
            }
            (None, Some(&(l, r))) => {
                if l == x + 1 {
                    self.set.remove(&(l, r));
                    self.set.insert((x, r));
                } else {
                    self.set.insert((x, x + 1));
                }
            }
            (None, None) => {
                self.set.insert((x, x + 1));
            }
        }
        true
    }

    // x以上のmex(minimum excludant (minimum excluded))
    pub fn mex(&self, x: i64) -> i64 {
        match self.set.range((i64::MIN, x)..).next() {
            Some(&(l, r)) => {
                if l > x {
                    x
                } else {
                    r
                }
            }
            None => x,
        }
    }
}