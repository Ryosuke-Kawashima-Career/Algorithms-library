use proconio::input;
// ABC440D
// Q. Mex(Minimum Excluded value) = the minimum positive integer that is not in the array => The problem generalized to Yth Mex
// A. Binary Search
fn main() {
    input! {n: usize, q: usize, mut a: [i64; n], xy: [(i64, i64); q]}
    // ソートで問題の条件を整理する．∵一般性は失われない．
    a.sort();
    // sentinal values(番兵) -INF, INF

    for &(x, y) in xy.iter() {
        // Calculate how many numbers are missing before x (i.e., in range [1, x) )
        // valid numbers in [1, x) = (x - 1)
        // present in A in [1, x) = partition_point < x
        let idx_x = a.partition_point(|&ai| ai < x);
        let missing_before_x = (x - 1) - idx_x as i64;

        // We want the y-th missing number >= x.
        // This is equivalent to the (missing_before_x + y)-th missing number overall.
        // 数値Xによってずらされる分を補正するのか...
        let k_th_missing = missing_before_x + y;

        // We need to find the specific number V such that exactly k_th_missing numbers are missing in [1, V].
        // Let C be the count of elements in A <= V.
        // Then (V - C) = k_th_missing  =>  V = k_th_missing + C.
        // We search for the value of C.
        // C will be the index in A such that A[C-1] is the largest element <= V.
        // Condition: The count of missing numbers strictly below A[i] is (A[i] - 1) - i.
        // If (A[i] - 1 - i) < k_th_missing, then A[i] must be less than the answer V.

        // Binary search for the smallest index where (a[i] - i - 1) >= k_th_missing
        let mut low = 0;
        let mut high = n;
        while low < high {
            let mid = (low + high) / 2;
            let missing_count_below_ai = a[mid] - (mid as i64) - 1;
            if missing_count_below_ai < k_th_missing {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        // low is the number of elements in A that are smaller than the answer
        let ans = k_th_missing + low as i64;
        println!("{}", ans);
    }
}
