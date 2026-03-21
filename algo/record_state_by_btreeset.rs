use proconio::{input, marker::{Chars, Usize1}};
// abc341E: record state by BTreeSet
// prefixの代わりに二分木による二分探索
fn main() {
    input!{n: usize, q: usize, s: Chars}
    // bad_indexes(i): s[i] vs. s[i+1](i: 0..n-1)
    let mut bad_indexes = std::collections::BTreeSet::new();
    for i in 0..n-1 {
        if s[i] == s[i+1] {
            bad_indexes.insert(i);
        }
    }
    
    let mut answers = Vec::new();
    for _ in 0..q {
        input!{query: usize, l: Usize1, r: Usize1}
        if query == 1 {
            // 変更する部分の端のみが変化する
            if l >= 1 {
                if bad_indexes.contains(&(l-1)) {
                    bad_indexes.remove(&(l-1));
                } else {
                    bad_indexes.insert(l-1);
                }
            }
            if r < n - 1 {
                if bad_indexes.contains(&r) {
                    bad_indexes.remove(&r);
                } else {
                    bad_indexes.insert(r);
                }
            }
        } else {
            // upper bound
            if let Some(&bad_index) = bad_indexes.range(l..).next() {
                if bad_index >= r {
                    answers.push("Yes");
                } else {
                    answers.push("No");
                }
            } else {
                answers.push("Yes");
            }
        }
    }

    for ans in answers.iter() {
        println!("{}", ans);
    }
}