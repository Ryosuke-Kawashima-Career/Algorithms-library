use proconio::input;
// abc430E: まずは全探索 + upper-lower-bound + 実験
// Q. How many rotations of String: A are at least needed to match it to String: B?
// Point: 文字列の左シフトであるため、 ∣A∣ 回操作をすると文字列が元に戻るので、 ∣A∣ 回以上の操作を行う必要はありません。
// A. Z-algorithm (B,A,A をこの順に連結した文字列 S を Z-algorithm にかけ、解答である Z-array z を取得)
// Aを二回繰り返しているのは円環を表しているのでは?
// Eg. (A="ABCDEABCD", B="pqrst") => Z="pqrstABCDEABCD"
// LCP(Longest Common Prefix)
fn main() {
    input!{t: usize, cases: [String; 2*t]}
    'outer: for case in 0..t {
        let a: String = cases[2*case].clone();
        let b: String = cases[2*case+1].clone();
        let c: Vec<char> = format!("{0}|{1}{1}", b, a).chars().collect();
        let longest_common_prefix = z_algorithm(&c);
        // bの長さ + '|' の長さ1 => length of B + 1
        for i in b.len()+1..longest_common_prefix.len() {
            if longest_common_prefix[i] == b.len() {
                println!("{}", i - 1 - b.len());
                continue 'outer;
            }
        }
        println!("-1");
    }
}

// 各 `i` について、`data[..]` と `data[i..]` の最長共通接頭辞の長さを O(N) で求めるアルゴリズム
fn z_algorithm(s: &Vec<char>) -> Vec<usize> {
    /*
    Compute the Z-array for a given string `s`.
    Z[i] is the length of the longest substring starting from `i` that matches the prefix of `s`.
    */
    let n: usize = s.len();
    let mut z: Vec<usize> = vec![0; n];
    // data == data[0..] <= self evident
    z[0] = n;
    let mut left: usize = 1;
    let mut length: usize = 0;
    
    while left < n {
        while left + length < n && s[length] == s[left+length] {
            length += 1;
        }
        z[left] = length;
        // 最長共通接頭辞の長さが 0 の場合、そのまま次の i について処理を始める
        if length == 0 {
            left += 1;
            continue;
        }

        // k: length of substrings
        let mut k: usize = 1;
        // `data[i..i+j]` は `data[0..j]` と共通であるから、1 以上の k について `i+k..i+result[k]` が `i..i+j` の範囲内に含まれるなら`result[k]` がそのまま答えになる。
        // 区間DPの漸化式の応用みたい
        while left + k < n && k + z[k] < length {
            z[left + k] = z[k];
            k += 1;
        }
        // 再利用した箇所はもう計算する必要がない。
        left += k;
        // 少なくとも j - k 文字は一致する。
        length -= k;
    }
    return z;
}