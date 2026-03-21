use proconio::input;
const MOD: usize = 100_000;
const N1: usize = 1usize.wrapping_neg();
// 典型58
// クエリの回数が大きいので周期性に注目する。
// 同じ値をもう一度探索したときが、一周!!!
fn main() {
    input!{mut x: usize, k: usize}
    let mut order: Vec<usize> = vec![N1; MOD];
    let mut cycle: Vec<usize> = Vec::new();
    let mut current_order: usize = 0;

    // if x is not visited
    while order[x] == N1 {
        order[x] = current_order;
        cycle.push(x);
        current_order += 1;

        let y: usize = digit_sum(x);
        let z: usize = (x + y) % MOD;
        x = z;
    }

    let cycle_start: usize = order[x];
    let period: usize = cycle.len() - cycle_start;
    let k_equiv: usize = if k >= cycle_start {
        (k - cycle_start) % period + cycle_start
    } else {
        k
    };
    let ans: usize = cycle[k_equiv];
    println!("{}", ans);
}

fn digit_sum(mut num: usize) -> usize {
    let mut res: usize = 0;
    while num > 0 {
        res += num % 10;
        num /= 10;
    }

    return res;
}