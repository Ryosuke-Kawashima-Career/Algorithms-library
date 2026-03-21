use proconio::input;
// ABC442D: Swap and Range Sum
// Q. Swap the value of i=x and i=x+1 and then Calculate the sum of [l r].
// A. Binary Indexed Tree or Prefix Sum
// 累積和を計算した後に配列の値を少し変えるだけで済むことに着目する．
fn main() {
    input! {n: usize, q: usize, mut a: [i64; n]}
    let mut prefix = get_prefix(&a);
    for _query in 0..q {
        input! {query_type: usize}
        if query_type == 1 {
            input! {x: usize}
            let value_x: i64 = a[x - 1];
            let value_x_plus_1: i64 = a[x];
            a[x - 1] = value_x_plus_1;
            a[x] = value_x;
            // prefix[x+1] is constant!!!
            prefix[x] = prefix[x] - value_x + value_x_plus_1;
        } else {
            input! {l: usize, r: usize}
            let ans: i64 = prefix[r] - prefix[l - 1];
            println!("{}", ans);
        }
    }
}

fn get_prefix(array: &[i64]) -> Vec<i64> {
    let mut prefix = vec![0; array.len() + 1];
    for i in 0..array.len() {
        prefix[i + 1] = prefix[i] + array[i];
    }
    prefix
}
