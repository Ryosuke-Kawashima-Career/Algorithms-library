use std::collections::HashSet;
fn main() {
    // vector -> hashset
    let num_set: HashSet<usize> = nums.iter().cloned().collect();
    // 集合の演算
    num_set = num_set.union(&num_set2).copied().collect();
    num_set = num_set.intersection(&num_set2).copied().collect();
}