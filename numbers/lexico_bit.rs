use proconio::input;
// 典型2
// 制約が小さいので全探索する。
// bitの桁を逆からfor文で回すことで、()の組み合わせが辞書順になる。
fn main() {
    input!{n: usize}
    if n % 2 == 1 {
        return;        
    }
    let mut bits: usize = 1 << n;

    // 小さい数が辞書順になる。(: 0, ): 1
    for bit in 0..bits {
        if is_ok(bit, n) {
            print_bit(bit, n);
        }
    }
}

fn is_ok(bit: usize, n: usize) -> bool {
    let limit: usize = n / 2;
    let mut l_num: usize = 0;
    let mut r_num: usize = 0;

    // 上の桁から見る。
    for i in (0..n).rev() {
        if bit >> i & 1 == 0 {
            l_num += 1;
        } else {
            r_num += 1;
        }

        if l_num < r_num {
            return false;
        }

        if l_num > limit || r_num > limit {
            return false;
        }
    }

    return true;
}

fn print_bit(bit: usize, n: usize) {
    let mut res = String::new();
    for i in (0..n).rev() {
        if bit >> i & 1 == 0 {
            res = format!("{}(", res);
        } else {
            res = format!("{})", res);
        }
    }

    println!("{}", res);
}