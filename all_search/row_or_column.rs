use proconio::input;
// JOI第7回E(おせんべい)
// rの制約が小さい -> rをbit全探索
fn main() {
    input!{r: usize, c: usize, state: [[usize; c]; r]}
    let mut ans: usize = 0;
    for bit in 0..(1 << r) {
        let mut senbei = state.clone();
        let mut cnt: usize = 0;
        for i in 0..r {
            for j in 0..c {
                // 1. 行で裏返す
                if bit >> i & 1 == 1 {
                    senbei[i][j] ^= 1;
                }
            }
        }

        // 2. 列で裏返す
        for j in 0..c {
            let mut cnt0: usize = 0;
            let mut cnt1: usize = 0;
            for i in 0..r {
                if senbei[i][j] == 0 {
                    cnt0 += 1;
                } else {
                    cnt1 += 1;
                }
            }

            cnt += cnt0.max(cnt1);
        }

        ans = ans.max(cnt);
    }

    println!("{}", ans);
}