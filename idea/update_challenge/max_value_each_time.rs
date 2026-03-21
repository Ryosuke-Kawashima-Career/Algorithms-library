use proconio::input;
// BinaryHeapを使う前にabc329d(典型63でも使う)
// 各時点における最大値とその持ち主の求めかた
fn main() {
    input!{n: usize, m: usize, challengers: [usize; m]}
    let mut votes: Vec<usize> = vec![0; n+1];

    let mut max_vote: usize = 0;
    let mut champion: usize = 0;
    for i in 0..m {
        let challenger: usize = challengers[i];
        votes[challenger] += 1;
        let vote: usize = votes[challenger];
        
        // 値が更新される時に最大値を名乗る資格がある!!!
        if vote > max_vote {
            max_vote = vote;
            champion = challenger;
        } else if vote == max_vote {
            // elseがないとifの後にこの処理をしてしまう。
            if champion > challenger {
                champion = challenger;
            }
        }

        println!("{}", champion);
    }
}