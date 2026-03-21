use proconio::input;
use proconio::marker::Chars;
// abc324c
// 前と後ろから比較する
// 文字列の長さで比較する
fn main() {
    input!{n: usize, target: Chars, s: [Chars; n]}
    let mut ans: Vec<usize> = Vec::new();
    for i in 0..n {
        if is_equiv(&s[i], &target) {
            ans.push(i+1);
        }
    }
    let k: usize = ans.len();
    println!("{}", k);
    for i in 0..k {
        print!("{} ", ans[i]);
    }
}

fn is_equiv(checked: &Vec<char>, target: &Vec<char>) -> bool {
    let n: usize = target.len();
    let m: usize = checked.len();
    // equal(same_front = same_back = n = m)
    if n == m {
        let mut is_all_same = true;
        for i in 0..n {
            if checked[i] != target[i] {
                is_all_same = false;
            }
        }
        if is_all_same {
            return true;
        }
    }
    
    // insert(checked > target)
    if m == n + 1 {
        // 一致する長さを前と後ろから調べる
        let mut same_front: usize = 0;
        let mut same_back: usize = 0;
        for i in 0..n {
            if checked[i] == target[i] {
                same_front += 1;
            } else {
                break;
            }
        }
        for i in 0..n {
            if checked[m-i-1] == target[n-i-1] {
                same_back += 1;
            } else {
                break;
            }
        }

        // 短いほう以上
        if same_front + same_back >= n {
            return true;
        }
    }

    // remove(checked < target)
    if m + 1 == n {
        // 一致する長さを前と後ろから調べる
        let mut same_front: usize = 0;
        let mut same_back: usize = 0;
        for i in 0..m {
            if checked[i] == target[i] {
                same_front += 1;
            } else {
                break;
            }
        }
        for i in 0..m {
            if checked[m-i-1] == target[n-i-1] {
                same_back += 1;
            } else {
                break;
            }
        }

        // 短いほう以上
        if same_front + same_back >= m {
            return true;
        }
    }

    // switch(same_front + same_back = n - 1 = m - 1)
    if n == m {
        let mut difs = 0;
        let mut is_ok = true;
        for i in 0..n {
            if target[i] != checked[i] {
                difs += 1;
            }
        }
        if difs > 1 {
            is_ok = false;
        }
        if is_ok {
            return true;
        }
    }
    return false;
}