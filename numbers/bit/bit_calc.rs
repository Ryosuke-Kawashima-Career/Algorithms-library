fn main() {
    // i桁が立っているか(n: bitの桁数)
    for i in 0..n {
        if bit >> i & 1 == 1 {}
        // bitがintの時は()をつけて演算する
        if (bit as i64 >> i as i64) & 1 == 1 {}
    }

    // i桁のbitを立てる。
    bit |= 1 << i;

    // i,j,k桁のbitを反転させる。
    // switchのon-offの反転をxor(1+0=1, 1+1=0, 0+0=0: 繰り上がりが消える足し算)で表現する。
    let next_bit: usize = bit ^ (1 << i) ^ (1 << j) ^ (1 << k);
}

fn popcount(mut n: usize) -> usize {
    let mut ones = 0;
    while n > 0 {
        ones += n & 1;
        n >>= 1;
    }
    return ones;
}

// most significant bit
fn msb(bit: usize) -> usize {
    const N: usize = 64;
    for digit in (0..N).rev() {
        if bit >> digit & 1 == 1 {
            return digit;
        }
    }
    return 0;
}