use proconio::input;
const INF: usize = 1 << 60;
// abc344D
// min cost = dp[item][state [start end)]
fn main() {
    input!{t: String, n: usize}
    let length: usize = t.len();
    let mut s: Vec<Vec<String>> = vec![vec![]; n];
    let mut a: Vec<usize> = Vec::new();
    // matched[i: 袋][j: 文字列][k: tの[start, end)と一致]
    let mut matched: Vec<Vec<Vec<(usize, usize)>>> = Vec::new();
    input_data(&t, n, &mut s, &mut a, &mut matched);
    
    // dp[i: どの袋まで使った][k: tの何文字目まで一致した?] 
    let mut dp: Vec<Vec<usize>> = vec![vec![INF; length + 1]; n+1];
    dp[0][0] = 0;
    for i in 1..=n {
        // 何もしない
        for k in 0..=length {
            dp[i][k] = dp[i][k].min(dp[i-1][k]);
        }
        for j in 0..a[i-1] {
            for &(start, end) in matched[i-1][j].iter() {
                // 漸化式をもとに式をたてる
                dp[i][end] = dp[i][end].min(dp[i-1][start] + 1);
            }
        }
    }

    if dp[n][length] == INF {
        println!("-1");
    } else {
        println!("{}", dp[n][length]);
    }
}

fn detect_pattern(part: &str, target: &str) -> Vec<(usize, usize)> {
    let part: Vec<char> = part.to_string().chars().collect();
    let target: Vec<char> = target.to_string().chars().collect();
    let part_len: usize = part.len();
    let target_len: usize = target.len();
    let mut res: Vec<(usize, usize)> = Vec::new();
    // targetとpartを比較する始点を全探索
    for start in 0..target_len {
        let mut is_ok = true;
        // targetとpartが一致するかpartの長さ分探索する
        for offset in 0..part_len {
            if start + offset >= target_len {
                is_ok = false;
                break;
            }
            if part[offset] != target[start + offset] {
                is_ok = false;
            }
        }
        if is_ok {
            // [start, start+part_len)
            res.push((start, start + part_len));
        }
    }

    return res;
}

// t: 対象の文字列, s: 文字列をいれる袋, a: 文字列の個数, matched: 文字列が一致するindex[start end)
fn input_data(t: &str, n: usize, s: &mut Vec<Vec<String>>,
    a: &mut Vec<usize>, matched: &mut Vec<Vec<Vec<(usize, usize)>>>) 
{
    for i in 0..n {
        input!{ai: usize}
        a.push(ai);
        let mut matched_i: Vec<Vec<(usize, usize)>> = Vec::new();
        for _ in 0..ai {
            input!{letters: String}
            let matched_indexes: Vec<(usize, usize)> = detect_pattern(&letters, t);
            s[i].push(letters);
            matched_i.push(matched_indexes);
        }
        matched.push(matched_i);
    }
}