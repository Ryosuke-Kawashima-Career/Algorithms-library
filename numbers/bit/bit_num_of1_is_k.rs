// n桁のbitのうち1の数がkのものを列挙する。
fn main() {
    let n: usize = 3;
    let k: usize = 2;
    
    let mut bit: usize = (1 << k) - 1;
    while bit < (1 << n) {
        bit = next_bit(bit);
    }
}

fn next_bit(bit: usize) -> usize {
    let sub: i64 = bit as i64;
    let x: i64 = sub & (-sub);
    let y: i64 = x + sub;
    let res: i64 = (((sub & !y) / x) >> 1) | y;
    return res as usize;
}