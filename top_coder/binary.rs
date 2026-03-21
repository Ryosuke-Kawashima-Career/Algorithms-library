pub trait BinarySearch<T> {
    fn lower_bound(&self, key: &T) -> usize;
    fn upper_bound(&self, key: &T) -> usize;
}

impl<T> BinarySearch<T> for [T]
where
    T: Ord,
{
    fn lower_bound(&self, key: &T) -> usize {
        let mut ng = -1_isize;
        let mut ok = self.len() as isize;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if *key <= self[mid as usize] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }

    fn upper_bound(&self, key: &T) -> usize {
        let mut ng = -1_isize;
        let mut ok = self.len() as isize;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if *key < self[mid as usize] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }
}

pub fn compress<T>(v: &[T]) -> Vec<usize>
where
    T: Ord + Clone,
{
    let mut vs = v.to_owned();
    vs.sort();
    vs.dedup();

    let mut res = vec![];
    for a in v {
        res.push(vs.lower_bound(a));
    }

    res
}