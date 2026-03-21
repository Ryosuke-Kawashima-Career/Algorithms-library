use proconio::{input, marker::Usize1};
const INF: usize = 1 << 60;
// JOI 2017 予選 4 - ぬいぐるみの整理
// 全探索: O(n!) -> bitDP: O(2^n)に改善する
// ぬいぐるみを入れ替えるので一つのぬいぐるみでなく，ぬいぐるみの種類をみる
fn main() {
    input!{n: usize, m: usize, a: [Usize1; n]}
    // 種類jのiまでの累積和 = dp[i: index][j: kind]
    let mut prefix: Vec<Vec<usize>> = vec![vec![0; m]; n+1];
    for i in 1..=n {
        for j in 0..m {
            if a[i-1] == j {
                prefix[i][j] = prefix[i-1][j] + 1;
            } else {
                prefix[i][j] = prefix[i-1][j];
            }
        }
    }

    // 種類の集合で探索する
    let bits: usize = 1 << m;
    let mut dp: Vec<usize> = vec![INF; bits];
    // 何もないとき，並び替えのコストは0
    dp[0] = 0;
    for bit in 0..bits {
        // bitに含まれる種類のぬいぐるみの個数の合計
        let mut length: usize = 0;
        for j in 0..m {
            if bit >> j & 1 == 1 {
                length += prefix[n][j];
            }
        }
        for j in 0..m {
            // jがbitに含まれていないとき
            if bit >> j & 1 == 0 {
                let next_state: usize = bit | (1 << j);
                // next_stateに含まれる種類のぬいぐるみの個数の合計
                let next_length: usize = length + prefix[n][j];
                dp[next_state] = dp[next_state].min(
                    dp[bit] + prefix[n][j] 
                    - prefix[next_length][j] + prefix[length][j]
                );
            }
        }
    }
    println!("{}", dp[bits-1]);
}