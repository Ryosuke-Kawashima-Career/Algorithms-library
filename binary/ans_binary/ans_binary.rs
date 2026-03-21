use proconio::input;
// 典型1
// 答えの二分探索: 答えになるかならないかの強化がわかる。
// 答えを決めれば、条件分岐できるときに二分探索は有効
fn main() {
    input!{n: usize, l: usize, k: usize, a: [usize; n]}
    let mut blocks: Vec<usize> = Vec::new();
    for i in 0..=n {
        if i == 0 {
            blocks.push(a[i]);
        } else if i == n {
            blocks.push(l - a[i-1]);
        } else {
            blocks.push(a[i]-a[i-1]);
        }
    }

    let mut length_l: usize = 0;
    // ピースがk以下
    let mut length_r: usize = l;
    while length_r - length_l > 1 {
        let mid: usize = (length_l+length_r) / 2;
        if calc_blocks(mid, &blocks) <= k {
            length_r = mid;
        }  else {
            length_l = mid;
        }
    }

    println!("{}", length_l);
}

fn calc_blocks(min_length: usize, lengths: &Vec<usize>) -> usize {
    let mut res: usize = 0;
    let mut cur_length: usize = 0;
    for &length in lengths.iter() {
        cur_length += length;
        if cur_length >= min_length {
            res += 1;
            cur_length = 0;
        }
    }

    return res;
}