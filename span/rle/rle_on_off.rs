use proconio::input;
// JOI 2013 本選 1 - 電飾
// 交代列に関して座標圧縮する
// 交代列 010101 101010: 値をxorで反転すれば交代列同士を接続できる
fn main() {
    input!{n: usize, c: [usize; n]}
    // 交代列の長さ
    let coord: Vec<usize> = compress(&c);
    // 接続する交代列の数
    let streak: usize = coord.len().min(3);
    let mut ans: usize = 0;
    for i in 0..n {
        let mut length: usize = 0;
        for di in 0..streak {
            if i + di >= coord.len() {
                continue;
            }
            length += coord[i+di];
        }
        ans = ans.max(length);
    }
    println!("{}", ans);
}

fn compress(c: &Vec<usize>) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    let n: usize = c.len();

    let mut i: usize = 0;
    while i < n {
        // 長さは必ず1以上
        let mut j: usize = i+1;
        while j < n && c[j-1] != c[j] {
            j += 1;
        }
        res.push(j-i);
        i = j;
    }

    return res;
}