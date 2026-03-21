use proconio::input;
use proconio::marker::Chars;
// 典型6
// 辞書順で最小のものを求めるときは、前から貪欲に決める。
fn main() {
    input!{n: usize, k: usize, s: Chars}
    // i以降にある文字cが, どの場所に初めて出現するか。next[i][c, j]
    let mut next: Vec<Vec<usize>> = vec![vec![n; 26]; n+1];
    // 逆から考える。
    for i in (0..n).rev() {
        for j in 0..26 {
            let c: usize = s[i] as usize - 'a' as usize;
            if c == j {
                next[i][j] = i;
            } else {
                next[i][j] = next[i+1][j];
            }
        }
    }

    let mut ans = String::new();
    let mut cur_index: usize = 0;
    while ans.len() < k {
        // 辞書順に小さい文字から確かめる。
        for j in 0..26 {
            // ちゃんと長さがkを超えられるか調べる。
            let possible_length: usize = ans.len() + n - next[cur_index][j];
            if possible_length >= k {
                ans = format!("{}{}", ans, char::from_u32(j as u32 + 'a' as u32).unwrap());
                cur_index = next[cur_index][j] + 1;
                // do not forget break!!!
                break;
            }
        }
    }

    println!("{}", ans);
}