use proconio::{input, marker::Usize1};
// abc310D
// t: チーム数, ab: 嫌いな人同士
// 嫌いな人同士が違うチームになるように分ける
// 選手を順にチームに追加する解法: O(N*N!)
fn main() {
    input!{n: usize, t: usize, m: usize, ab: [(Usize1, Usize1); m]}
    // i番目とj番目の仲が悪い = hate[i]のjbit目が1
    let mut hate: Vec<usize> = vec![0; n];
    for &(a, b) in ab.iter() {
        hate[a] |= 1 << b;
        hate[b] |= 1 << a;
    }
    // 人0,1,3がチームに所属するとき: 1011と記す
    let mut teams: Vec<usize> = Vec::new();
    let ans = dfs(0, t, &hate, &mut teams);
    println!("{}", ans);
}

// 再帰関数でチーム分けする
fn dfs(cur_person: usize, t: usize, hate: &Vec<usize>, teams: &mut Vec<usize>) -> usize {
    let n: usize = hate.len();
    // base case
    if cur_person == n {
        if teams.len() == t {
            return 1;
        }
        return 0;
    }
    let mut res: usize = 0;

    // 漸化式を立てる
    // すでにあるチームにcur_personを入れる
    for i in 0..teams.len() {
        // cur_personと仲の悪い人がいないとき
        if teams[i] & hate[cur_person] == 0 {
            teams[i] ^= 1 << cur_person;
            res += dfs(cur_person+1, t, hate, teams);
            // 帰りがけに復元する
            teams[i] ^= 1 << cur_person;
        }
    }
    // チーム数を増やす
    if teams.len() < t {
        teams.push(1 << cur_person);
        res += dfs(cur_person+1, t, hate, teams);
        teams.pop();
    }
    return res;
}