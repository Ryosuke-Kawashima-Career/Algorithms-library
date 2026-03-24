// Educational DP U
// Q. Divide the numbers {0..n} into some number of groups and maximize the sum of the minimums of each group.
// A. Bit Mask
/* Enumeration of sub groups
1 を引くことは、一番下の立っているビットを 0 にして、それより下のビットをすべて 1 にすることと同じである
O(3^N) ∵二項定理(∑2^n*nCk = 3^n)
 */
fn bit_subgroups<T>(array: &[T]) {
    let n: usize = array.len();
    let bits: usize = 1 << n;
    for mask in 0..bits {
        let mut sub_mask = (mask - 1) & mask;
        while sub_mask > 0 {
            if sub_mask == mask {
                continue;
            }
            sub_mask = (sub_mask - 1) & mask;
        }
    }
}
