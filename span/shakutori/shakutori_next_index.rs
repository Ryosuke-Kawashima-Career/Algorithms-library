use proconio::{input, marker::Chars};
// ABC430C
// Q. Combination of (l, r) 1≤l≤r≤N?
// The number of a in S[l..=r] is greater than or equal to A.
// The number of b in S[l..=r] is less than B.
// A. (1)Shakutori + (2)Next Index for each Index
fn main() {
    input!{n: usize, a: usize, b: usize, s: Chars}
    let prefix_a: Vec<usize> = prefix('a', &s);
    let prefix_b: Vec<usize> = prefix('b', &s);
    // limit_indexes[start_index][limit_index] = "a", "b" < a or b
    let limit_indexes_a: Vec<usize> = shakutori('a', a, &s);
    let limit_indexes_b: Vec<usize> = shakutori('b', b, &s);
    let mut ans: usize = 0;
    for start in 0..n {
        ans += limit_indexes_b[start].saturating_sub(limit_indexes_a[start]);
    }
    println!("{}", ans);
}

fn prefix(c: char, s: &Vec<char>) -> Vec<usize> {
    let n: usize = s.len();
    let mut prefix_c: Vec<usize> = vec![0; n+1];
    for i in 1..=n {
        if s[i-1] == c {
            prefix_c[i] = prefix_c[i-1] + 1;
        }
    }
    return prefix_c;
}

fn shakutori(target: char, limit_num: usize, s: &Vec<char>) -> Vec<usize> {
    let n: usize = s.len();
    let mut count: usize = 0;
    let mut r: usize = 0;
    let mut limit_indexes: Vec<usize> = (0..n).collect();
    for l in 0..n {
        while r < n {
            let add_num: usize = if s[r] == target {
                1
            } else {
                0
            };
            if count + add_num < limit_num {
                r += 1;
            } else {
                break;
            }
            count += add_num;
        }
        limit_indexes[l] = r;
        if s[l] == target {
            count -= 1;
        }
    }
    return limit_indexes;
}