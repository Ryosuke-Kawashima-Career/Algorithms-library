use proconio::input;
use proconio::marker::Chars;
// abc324d
// 平方数の√の全探索
// 平方数の要素の数が(s[perm[i]])と一致するか判定すればいい
fn main() {
    input!{n: usize, s: Chars}
    let s: Vec<usize> = s.iter().map(|&c| c as usize - '0' as usize).collect();
    // sに含まれる0..=9の要素の個数
    let mut nums: Vec<usize> = vec![0; 10];
    for i in 0..n {
        nums[s[i]] += 1;
    }
    let mut ans: usize = 0;
    // n桁の数なので10^n未満
    let max_num: usize = 10usize.pow(n as u32);
    let mut sqrt: usize = 0;

    // sqrtの全探索
    while sqrt * sqrt < max_num {
        let square: usize = sqrt * sqrt;
        let mut s_square: Vec<usize> = square.to_string().chars().map(|c| c as usize - '0' as usize).collect();
        if s_square.len() < n {
            for _ in 0..(n - s_square.len()) {
                s_square.push(0);
            }
        }
        let mut nums_square: Vec<usize> = vec![0; 10];
        for i in 0..s_square.len() {
            nums_square[s_square[i]] += 1;
        }
        if nums == nums_square {
            ans += 1;
        }
        sqrt += 1;
    }
    println!("{}", ans);
}