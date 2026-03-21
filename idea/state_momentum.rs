use proconio::{input};
// abc379c
// Q. how many times at least do you move one stone from i to i+1?
// A. express the current state with X (stones) * A (distance)
fn main() {
    input!{n: usize, m: usize, x: [usize; m], a: [usize; m]} 
    // momentum = weight * distance
    let mut momentum: Vec<(usize, usize)> = Vec::new();
    for i in 0..m {
        momentum.push((x[i], a[i]));
    }
    momentum.sort();
    // the final momentum = 1 * 1 + 1 * 2 + 1 * n
    let target_momentum: usize = n * (n + 1) / 2;
    let mut current_momentum: usize = 0;
    let mut stones_sum: usize = 0;
    for &(distance, stones) in momentum.iter() {
        // if the previous cells are not filled 
        if stones_sum < distance - 1 {
            println!("-1");
            return;
        }
        stones_sum += stones;
        current_momentum += distance * stones;
    }
    if stones_sum != n {
        println!("-1");
        return;
    }
    let ans = target_momentum - current_momentum;
    println!("{}", ans);
}