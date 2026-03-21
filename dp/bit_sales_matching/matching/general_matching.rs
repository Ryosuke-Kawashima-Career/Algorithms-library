use proconio::input;
// abc318d
// 全探索(ノードを２つずつ選び、エッジを選ぶ)
// bitDPで解く
fn main() {
    input!{n: usize}
    // 隣接行列を使用する
    let mut graph: Vec<Vec<usize>> = vec![vec![0; n]; n];
    for i in 0..n {
        for j in (i+1)..n {
            input!{dij: usize}
            graph[i][j] = dij;
            graph[j][i] = dij;
        }
    }
    let bits: usize = 1 << n;
    let mut dp: Vec<usize> = vec![0; bits];

    // ノードが..使われている: 1, 使われていない: 0
    for bit in 0.. bits {
        let mut start: usize = n;
        for i in 0..n {
            if bit >> i & 1 == 0 {
                start = i;
                break;
            }
        }
        if start == n {
            continue;
        }

        for i in start+1..n {
            // if not used
            if bit >> i & 1 == 1 {
                continue;
            }
            // startとiを使用するので
            let next_state: usize = bit | (1 << start) | (1 << i);
            dp[next_state] = dp[next_state].max(dp[bit] + graph[start][i]);
        }
    }

    println!("{}", dp[bits - 1]);
}