use proconio::input;
// 鉄則C5
fn main() {
    input!{n: usize}
    let mut luckies: Vec<usize> = Vec::new();
    enum_luck(0, &mut luckies);
    let luckies_bit: Vec<usize> = enum_luck_bit(10);
    println!("{}", luckies[n-1]);
}

// 4と7のみで構成される数を再帰関数で列挙(小さい順)
fn enum_luck(num: usize, luckies: &mut Vec<usize>) {
    // 10桁のi桁の数の条件 10^i-1 <= num < 10^i ex.10^9~10^10
    if 1_000_000_000 <= num && num < 10_000_000_000 {
        luckies.push(num);
        return;
    }

    enum_luck(10*num + 4, luckies);
    enum_luck(10*num + 7, luckies);
}

// 4と7のみで構成される数をbitで列挙(小さい順)
fn enum_luck_bit(digits: usize) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    // 0: 4, 1: 7
    for bit in 0..(1 << digits) {
        let mut num: usize = 0;
        // 上の桁から足していく -> 逆順で調べる
        for i in (0..digits).rev() {
            if bit >> i & 1 == 0 {
                num = 10 * num + 4; 
            } else {
                num = 10 * num + 7;
            }
        }

        res.push(num);
    }

    return res;
}