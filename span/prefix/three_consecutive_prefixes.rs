use proconio::input;
const INF: i64 = 1 << 60;
// ABC438-D
// Q. We want to maximize: Sum(A[1..x]) + Sum(B[x+1..y]) + Sum(C[y+1..N])
// A. This is equivalent to: (prefix_a[x] - prefix_b[x]) + (prefix_b[y] - prefix_c[y]) + prefix_c[n] = 方程式を展開することで式変形を行う．
// where 1 <= x < y < n.
// For a fixed y, we maximize (prefix_a[x] - prefix_b[x]) for 1 <= x < y.
// max_diff tracks max(prefix_a[x] - prefix_b[x]) for valid x so far
// Update max_diff for the next iteration where x can also be y
// 累積和が連続してつながっているのが要点！
fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
        c: [i64; n],
    }
    let prefix_a = get_prefix(&a);
    let prefix_b = get_prefix(&b);
    let prefix_c = get_prefix(&c);
    // prefix_c[n] is definitely included in the answer
    let mut ans = -INF;
    let mut max_diff_of_prefix_a_and_prefix_b = prefix_a[1] - prefix_b[1];
    for y in 2..n {
        // 計算の順序に注意する = maxの計算が後
        let cur_val = max_diff_of_prefix_a_and_prefix_b + prefix_b[y] - prefix_c[y] + prefix_c[n];
        ans = ans.max(cur_val);
        max_diff_of_prefix_a_and_prefix_b =
            max_diff_of_prefix_a_and_prefix_b.max(prefix_a[y] - prefix_b[y]);
    }
    println!("{}", ans);
}

fn get_prefix(array: &Vec<i64>) -> Vec<i64> {
    let n = array.len();
    let mut prefix = vec![0; n + 1];
    for i in 1..=n {
        prefix[i] = prefix[i - 1] + array[i - 1];
    }
    prefix
}
