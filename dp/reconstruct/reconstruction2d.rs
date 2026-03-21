use proconio::input;
// 部分和問題
// 鉄則B18
fn main() {
    input!{n: usize, s: usize, a: [usize; n]}
    // dp[i][j]: カードをi枚使ってj点を達成できるか？
    // 1indexed
    let mut dp: Vec<Vec<usize>> = vec![vec![0; s+1]; n+1];
    dp[0][0] = 1;
    for i in 1..=n {
        for j in 0..=s {
            // 何も選ばない。
            dp[i][j] += dp[i-1][j];
            if a[i-1] <= j {
                dp[i][j] += dp[i-1][j-a[i-1]];
            }
        }
    }
    
    let mut recon: Vec<usize> = Vec::new();
    // 上のfor文から初期値を決める。
    let mut card_index: usize = n;
    let mut score_index: usize = s;
    while card_index > 0 && score_index > 0 {
        // if文の順序に注意する。
        if dp[card_index][score_index] == dp[card_index-1][score_index] {
            card_index -= 1;
            continue;
        }
        if a[card_index-1] <= score_index 
        && dp[card_index][score_index] == dp[card_index-1][score_index] + dp[card_index-1][score_index-a[card_index-1]] {
            recon.push(card_index);
            score_index -= a[card_index-1];
            card_index -= 1;
            continue;
        }
    }

    let num: usize = recon.len();
    if num == 0 {
        println!("-1");
    } else {
        println!("{}", num);
        for i in 1..=num {
            print!("{} ", recon[num-i]);
        }
    }
}