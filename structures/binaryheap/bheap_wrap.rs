use proconio::input;
// abc329d
fn main() {
    input!{n: usize, m: usize, a: [usize; m]}
    let mut candidate_votes: Vec<i64> = vec![0; n+1];
    let mut bh = std::collections::BinaryHeap::new();
    for i in 0..m {
        candidate_votes[a[i]] += 1;
        bh.push(Idval::new(a[i], candidate_votes[a[i]]));
        let candidate = bh.peek().unwrap().id;
        println!("{}", candidate);
    }

    while let Some(Idval{id, val}) = bh.pop() {
    }
}

// low_priority.cmp(high_priority)
// self.cmp(other): self(small) < other(big)
// other.cmp(self): other(big) < self(small)

// idが小さくvalが大きいと優先度が高くなる。
#[derive(Debug, Copy, Clone, Eq)]
struct Idval {
    id: usize, val: i64
}

impl Idval {
    fn new(index: usize, value: i64) -> Self {
        Self{id: index, val: value}
    }
}

impl PartialOrd for Idval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.val == other.val {
            (other.id).partial_cmp(&self.id)
        } else {
            (self.val).partial_cmp(&other.val)
        }
    }
}
impl Ord for Idval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.val == other.val {
            (other.id).cmp(&self.id)
        } else {
            (self.val).cmp(&other.val)
        }
    }
}

impl PartialEq for Idval {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.val == other.val
    }
}