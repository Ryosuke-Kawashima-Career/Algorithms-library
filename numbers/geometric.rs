use proconio::input;
// abc413d
// Q. 等比数列の判定問題
// A. 公比r=1, -1, the others?
fn main() {
    input!{nt: usize}
    let mut answer: Vec<String> = Vec::new();
    for _test in 0..nt {
        input!{n: usize, mut a: [i64; n]}
        // なぜか降順で整列
        a.sort_by(|x, y| y.abs().cmp(&(x.abs())));
        
        if is_geometric(&a) {
            answer.push("Yes".to_string());
        } else {
            answer.push("No".to_string());
        }
    }

    for ans in answer.iter() {
        println!("{}", ans);
    }
}

fn is_geometric(a: &Vec<i64>) -> bool {
    let n: usize = a.len();
    // r = 1?
    let mut is_r_1: bool = true;
    for i in 1..n {
        if a[0] != a[i] {
            is_r_1 = false;
        }
    }

    // r = -1?
    let mut cnt_same: usize = 1;
    let mut cnt_opposite: usize = 0;
    for i in 1..n {
        if a[0] == a[i] {
            cnt_same += 1;
        }
        if a[0] == -a[i] {
            cnt_opposite += 1;
        }
    }
    let is_r_minus1 = (cnt_same + cnt_opposite == n && cnt_same.abs_diff(cnt_opposite) <= 1);

    // Regular cases
    let mut does_r_exist: bool = true;
    for i in 0..n-2 {
        if a[i] * a[i+2] != a[i+1] * a[i+1] {
            does_r_exist = false;
        }
    }

    return (is_r_1 || is_r_minus1 || does_r_exist);
}