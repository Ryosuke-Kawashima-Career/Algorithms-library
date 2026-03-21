use proconio::input;
// abc430E: まずは全探索 + upper-lower-bound + 実験
// Q. How many rotations of String: A are at least needed to match it to String: B?
// Point: 文字列の左シフトであるため、 ∣A∣ 回操作をすると文字列が元に戻るので、 ∣A∣ 回以上の操作を行う必要はありません。
// A.Knuth-Morris-Pratt (KMP): KMP relies on a prefix table (or partial match table) to avoid redundant comparisons. It’s particularly efficient for repetitive patterns in the search string.
// LCP(Longest Common Prefix)
fn main() {
    input!{t: usize, cases: [String; 2*t]}
    for case in 0..t {
        let a: String = cases[2*case].clone();
        let b: Vec<char> = cases[2*case+1].chars().collect();
        // Aを2回繰り返して、その中からBを探す
        // 見つかった位置がシフト回数になる
        let a_double: Vec<char> = format!("{0}{0}", a).chars().collect();
        let longest_common_prefix_a_doulbe: Vec<usize> = kmp_search(&a_double, &b);
        if let Some(position) = longest_common_prefix_a_doulbe.iter().next() {
            println!("{}", position);
        } else {
            println!("-1");
        }
    }
}

fn longest_common_prefix(pattern: &[char]) -> Vec<usize> {
    /*
    Compute the prefix table for the KMP algorithm.
    longest_prefix_suffix[i] stores the length of the longest prefix
    which is also a suffix for the substring pattern[0:i+1].
     */
    let m: usize = pattern.len();
    // lengest_prefix_suffix
    // Ex. "abab" longest_prefix_suffix = [0, 0, 1, 2]　<- "ab~" == "~ab"
    let mut longest_prefix_suffix: Vec<usize> = vec![0; m];
    // Length of the previous longest prefix suffix
    let mut prev_lcp: usize = 0;

    // You should ensure i starts at 1 !!!!!!!!!!!!!!!!!!!
    for i in 1..m {
        while prev_lcp > 0 && pattern[i] != pattern[prev_lcp] {
            prev_lcp = longest_prefix_suffix[prev_lcp - 1];
        }
        if pattern[i] == pattern[prev_lcp] {
            prev_lcp += 1;
        }
        longest_prefix_suffix[i] = prev_lcp;
    }
    return longest_prefix_suffix;
}

fn kmp_search(text: &[char], pattern: &[char]) -> Vec<usize> {
    /* Knuth-Morris-Pratt
    Search for all occurrences of `pattern` in `text` using the KMP algorithm.
     */
    let longest_prefix_suffix: Vec<usize> = longest_common_prefix(pattern);
    let n: usize = text.len();
    let m: usize = pattern.len();
    let mut occurrences: Vec<usize> = Vec::new();

    let mut i: usize = 0;
    let mut j: usize = 0;
    while i < n {
        if text[i] == pattern[j] {
            i += 1;
            j += 1;
        }
        if j == m {
            // when the pattern is found
            occurrences.push(i - j);
            j = longest_prefix_suffix[j-1];
        } else if i < n && text[i] != pattern[j] {
            if j > 0 {
                // patternの中でindex=jから前のindexに戻る
                j = longest_prefix_suffix[j-1];
            } else {
                i += 1;
            }
        }
    }

    return occurrences;
}