use proconio::{input, marker::Chars};
// abc441e
// Q. Count the number of Substrings comprised of 'A', 'B', 'C' such that 'A' is more than 'B''C'
// A. Equation Transformation
/* S の i 文字目から j 文字目までを取って得られる部分文字列が条件を満たすことは、Aj−Ai−1>Bj−Bi−1と言い換えることができます。これは式変形をすることで, Aj−Bj>Ai−1−Bi−1となります。 */
fn main() {
    input! {n: usize, s: Chars}
    // A: +1 B: -1 C: 0
    let mut sign: i64 = 0;
    // -n ~ n
    let offset: i64 = n as i64;
    let get_sign = |sign: i64| -> usize { (offset + sign) as usize };
    // counter: An array tracking how many times each sign value has appeared so far (including the initial 0).
    let mut counter: Vec<usize> = vec![0; 2 * n + 1];
    counter[get_sign(sign)] = 1;

    // 現在の D=A-B 未満の d に対する counter[d] の総和 = 転倒数と似た要領で求める．
    // sum: The number of valid starting points seen so far. A starting point is valid if its prefix sign is strictly less than the current sign.
    let mut sum: usize = 0;
    let mut substrings: usize = 0;
    for c in s {
        if c == 'A' {
            sum += counter[get_sign(sign)];
            sign += 1;
        } else if c == 'B' {
            sign -= 1;
            sum -= counter[get_sign(sign)];
        }
        counter[get_sign(sign)] += 1;
        // substrings: The total number of valid substrings seen so far.
        substrings += sum;
    }
    println!("{}", substrings);
}
