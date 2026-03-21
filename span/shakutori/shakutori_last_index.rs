use proconio::input;
// abc381D
// Q. 1. ∣X∣ は偶数である
// 2. 1≤i≤2∣X∣をみたす整数iについて、X2i−1とX2iは等しい。
// 3. 各正整数はXに現れないか、ちょうど2回現れるかのどちらかである。すなわち、Xに含まれる正整数はXにちょうど2回ずつ登場する。
// A. 1≤l<r≤N について、(Al​,Al+1,…,Ar)が1122数列であるとき、最後から2項取り除いた列、すなわち(Al,Al+1,…,Ar−2)も1122数列
// ここで、r−l=2 のときは取り除いた後の数列は空列となることに注意する。
// このことから尺取り法を用いてこの問題を解くことができます。
fn main() {
    input!{n: usize, a: [usize; n]}
    let mut ans: usize = 0;
    // (Al,Al+1,…,Ar)が1122数列となる最小の奇数l(1≤l≤r+1)をf(r)として定義
    let mut last_indexes_even: Vec<isize> = vec![-2; n+1];
    last_indexes_even[0] = 0;
    // 偶数から始まる
    let mut l: usize = 0;
    let mut index: usize = 0;
    while index + 1 < n {
        if a[index] != a[index+1] {
            l = index + 2;
        } else {
            l = l.max((last_indexes_even[a[index]] + 2) as usize);
        }
        ans = ans.max(index+2-l);
        last_indexes_even[a[index]] = index as isize;
        index += 2;
    }
    // 奇数から始まる
    let mut last_indexes_odd: Vec<isize> = vec![-2; n+1];
    last_indexes_odd[0] = 0;
    let mut l: usize = 1;
    let mut index: usize = 1;
    while index + 1 < n {
        if a[index] != a[index+1] {
            l = index + 2;
        } else {
            l = l.max((last_indexes_odd[a[index]] + 2) as usize);
        }
        ans = ans.max(index+2-l);
        last_indexes_odd[a[index]] = index as isize;
        index += 2;
    }

    println!("{}", ans);
}