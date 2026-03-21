use proconio::input;
// ABC435E
// Q. lr continues to come, paint [l, r) to black. After each query, output how many white cells remain.
// A. BTreeSet manages the ranges dynamically.
// we manage the black segements, unlike the model answer.
fn main() {
    input! {n: usize, q: usize, lr: [(i64, i64); q]}
    // initially all white
    let mut black_segments = IntervalSet::new();
    for query in 0..q {
        let (l, r) = lr[query];
        let l = l - 1; // [l, r)
        black_segments.cover(l, r);
        let white_segments = n - black_segments.len;
        println!("{}", white_segments);
    }
}

//　 IntervalSet that manages covered intervals
pub struct IntervalSet {
    set: std::collections::BTreeSet<(i64,i64)>,
    len: usize,
}
impl IntervalSet {
    pub fn new() -> Self {
        let mut set = std::collections::BTreeSet::new();
        set.insert((i64::MIN, i64::MIN));
        set.insert((i64::MAX, i64::MAX));
        Self{set, len: 0}
    }
    pub fn cover(&mut self, l:i64, r:i64) {
        let mut removes = vec![];
        let mut inserts = vec![(l, r)];
        //　左側境界
        let &(al, ar) = self.set.range(..(l, i64::MIN)).next_back().unwrap();
        for &(nl, nr) in self.set.range((al, ar)..){
            if nl < l && l < nr {
                removes.push((nl, nr));
                inserts.push((nl, l));
            }
            if r < nr {
                if nl < r {
                    removes.push((nl, nr));
                    inserts.push((r, nr));
                }
                break;
            }
            if l <= nl && nr <= r{
                removes.push((nl, nr));
            }
        }
        
        for &(dl, dr) in removes.iter(){
            if self.set.remove(&(dl, dr)){
                self.len -= (dr - dl) as usize;
            }
        }
        for &(dl, dr) in inserts.iter(){
            self.set.insert((dl, dr));
            self.len += (dr - dl) as usize;
        }
    }
    
}