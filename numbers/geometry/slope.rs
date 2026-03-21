use proconio::input;
use std::collections::HashMap;
// Q. What is the number of 台形?
// A. ダブりを消去する(Off-set the overlap) + Calculate the slopes by ModInt
// A. 平行四辺形であるような台形は2回，そうでない台形は1回数えられている．したがって，平行四辺形の個数を答えから引く必要がある．
// A. 2つの対角線の中点が一致することが平行四辺形であることと同値
// A. 中点がyであるような2点の選び方の個数をdyとする．このとき，dy*(dy−1)/2を答えから引く．
fn main() {
    input!{n: usize, xy: [(i64, i64); n]}
    let count_slopes: HashMap<(i64, i64), usize> = calc_slopes(&xy);
    let count_centers: HashMap<(i64, i64), usize> = calc_centers(&xy);
    let ans: usize = count_pairs(&count_slopes) - count_pairs(&count_centers);
    println!("{}", ans);
}

fn calc_slopes(coord: &Vec<(i64, i64)>) -> HashMap<(i64, i64), usize> {
    let n: usize = coord.len();
    let mut count_slopes: HashMap<(i64, i64), usize> = HashMap::new();
    for p1 in 0..n {
        for p2 in p1+1..n {
            let mut dx = coord[p2].0 - coord[p1].0;
            let mut dy = coord[p2].1 - coord[p1].1;
            if dx == 0 {
                dx = 0;
                dy = 1;
            } else if dy == 0 {
                dx = 1;
                dy = 0;
            } else {
                if dx < 0 {
                    dx *= -1;
                    dy *= -1;
                }
                let gcd_dx_dy: i64 = gcd(dx.abs(), dy.abs());
                dx /= gcd_dx_dy;
                dy /= gcd_dx_dy;
            }
            *count_slopes.entry((dx, dy)).or_insert(0) += 1;
        }
    }
    return count_slopes;
}

// Do not divide by 2!! (4/2 == 5/2)
fn calc_centers(coord: &Vec<(i64, i64)>) -> HashMap<(i64, i64), usize> {
    let n: usize = coord.len();
    let mut count_centers: HashMap<(i64, i64), usize> = HashMap::new();
    for p1 in 0..n {
        for p2 in p1+1..n {
            let x_mid = coord[p2].0 + coord[p1].0;
            let y_mid = coord[p2].1 + coord[p1].1;
            *count_centers.entry((x_mid, y_mid)).or_insert(0) += 1;
        }
    }
    return count_centers;
}

fn count_pairs(counter: &HashMap<(i64, i64), usize>) -> usize {
    let mut pairs: usize = 0;
    for (_, &num) in counter.iter() {
        if num > 0 {
            pairs += num * (num - 1) / 2;
        }
    }
    return pairs;
}

fn gcd(n: i64, m: i64) -> i64 {
    if n == 0 || m == 0 {
        return n + m;
    }
    return gcd(m, n % m);
}