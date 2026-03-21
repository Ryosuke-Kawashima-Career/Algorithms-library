use proconio::input;
// abc367d
// 湖の周りにN個の休憩所があります。休憩所には時計回りに1,2,…,N の番号が付いています。
// 休憩所iから休憩所i+1まで時計回りに歩くにはAi歩かかります(但し休憩所N+1は休憩所1を指します)
// 休憩所sから休憩所t(s!=t) まで時計回りに歩くのにかかる最小の歩数はMの倍数でした。
// (s,t)の組として考えられるものの数は?
fn main() {
    input!{n: usize, m: usize, a: [usize; n]}
    // 2*nで円環を表す
    let mut prefix: Vec<usize> = vec![0; 2*n+1];
    for i in 1..=2*n {
        prefix[i] = prefix[i-1] + a[(i - 1) % n];
    }
    // modで分類する
    let mut cnt_mod: Vec<usize> = vec![0; m];
    // 1周目
    for i in 0..n {
        cnt_mod[prefix[i] % m] += 1;
    }
    let mut ans: usize = 0;
    // 2週目
    for i in n..2*n {
        cnt_mod[prefix[i-n] % m] -= 1;
        ans += cnt_mod[prefix[i] % m];
        cnt_mod[prefix[i] % m] += 1;
    }
    println!("{}", ans);
}