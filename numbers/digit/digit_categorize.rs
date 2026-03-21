use proconio::input;
// ABC433D
// Q. f(12,3)=123 What are the combinations of (i, j) s.t. f(i,j) % m = 0?
// A. iとAjの**桁数kを固定**した時に条件を全て満たすjの個数を数える
// => Binary Search (Lower Bound = the number of no more than target_mod)
fn main() {
    input! {n: usize, m: usize, a: [usize; n]}
    let digits_for_mods_sorted: Vec<Vec<usize>> = categorize_by_digits_mod(m, &a);
    let mut ans: usize = 0;
    for i in 0..n {
        for digits in 1..=10 {
            let ai_mul_pow10 = ((a[i] % m) * (10usize.pow(digits as u32) % m)) % m;
            // Do not forget to mod 'm' here
            let target_mod: usize = (m - ai_mul_pow10) % m;
            let count_r: usize =
                digits_for_mods_sorted[digits].partition_point(|&x| x < target_mod + 1);
            let count_l: usize =
                digits_for_mods_sorted[digits].partition_point(|&x| x < target_mod);
            ans += count_r - count_l;
        }
    }
    println!("{}", ans);
}

fn categorize_by_digits_mod(m: usize, a: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut digits_for_mods_sorted: Vec<Vec<usize>> = vec![Vec::new(); 11];
    for &ai in a {
        let digits: usize = ai.to_string().len();
        digits_for_mods_sorted[digits].push(ai % m);
    }
    for v in digits_for_mods_sorted.iter_mut() {
        v.sort();
    }
    return digits_for_mods_sorted;
}
