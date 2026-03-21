use proconio::input;
const INF: usize = 1 << 60;
// reconstruction dp
// 鉄則A17
fn main() {
    input!{n: usize, a: [usize; n-1], b: [usize; n-2]}
    let mut dp: Vec<usize> = vec![INF; n];
    dp[0] = 0;
    for i in 0..n {
        if i >= 1 {
            dp[i] = dp[i].min(dp[i-1] + a[i-1]);
        }
        if i >= 2 {
            dp[i] = dp[i].min(dp[i-2] + b[i-2]);
        }
    }

    let mut recon: Vec<usize> = Vec::new();
    // 後ろからさかのぼる。
    // 上のfor文から初期値を決める。
    let mut index: usize = n-1;
    recon.push(n-1);
    while index > 0 {
        if index >= 1 && dp[index] == dp[index-1] + a[index-1] {
            recon.push(index-1);
            index -= 1;
            continue;
        }
        if index >= 2 && dp[index] == dp[index-2] + b[index-2] {
            recon.push(index-2);
            index -= 2;
            continue;
        }
    }

    let k: usize = recon.len();
    println!("{}", k);
    for i in 1..=k {
        print!("{} ", recon[k-i]+1);
    }
}