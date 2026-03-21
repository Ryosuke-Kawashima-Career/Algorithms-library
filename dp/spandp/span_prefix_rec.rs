use proconio::input;
const INF: usize = 1 << 60;
// educational DP N: Slimes
// 区間Dp: 累積和で高速に計算する
fn main() {
    input!{n: usize, a: [usize; n]}
    // min cost[l r) = dp[l][r]
    let mut dp: Vec<Vec<usize>> = vec![vec![INF; n+1]; n+1];
    // 累積和
    let mut prefix: Vec<usize> = vec![0; n+1];
    for i in 1..=n {
        prefix[i] = prefix[i-1] + a[i-1];
    }
    
    println!("{}", memo_rec(0, n, &prefix, &mut dp));
}

fn memo_rec(l: usize, r: usize, prefix: &Vec<usize>, dp: &mut Vec<Vec<usize>>) -> usize {
    // if visited return
    if dp[l][r] != INF {
        return dp[l][r];
    }
    // base case(need check!!!)
    if l + 1 == r {
        return 0;
    }
    
    let mut res = INF;
    // k: 中継点
    for k in l+1..r {
        // 自分自身の長さがコストになる
        let cost_k = memo_rec(l, k, prefix, dp) + memo_rec(k, r, prefix, dp) 
            + prefix[r] - prefix[l];
        res = res.min(cost_k);
    }

    // memo and return
    dp[l][r] = res;
    return res;
}