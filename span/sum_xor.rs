use proconio::input;
// abc121d
// Q. a ^ a+1 ... ^ b-1 ^ b
// A. n ^ n+1 == 1
// A. 同じ数をxorすると打ち消しあって0になる
fn main() {
    input!{a: usize, b: usize}
    let xor_sum = |n: usize| -> usize {
        match n % 4 {
            0 => n,
            1 => 1,
            2 => n + 1,
            3 => 0,
            _ => unreachable!()
        }
    };
    let ans = if a == 0 {
        xor_sum(b)
    } else {
        xor_sum(a-1) ^ xor_sum(b)
    };
    println!("{}", ans);
}