use proconio::input;
// abc412C
// Q. domino倒し
// A. 出来る限り大きなdominoを選ぶ
fn main() {
    input!{t: usize}
    for _ in 0..t {
        input!{n: usize, s: [usize; n]}
        // greedy algorithm but it does not need sorting (Snow Ball Effect)
        // <- 毎回できるだけ大きいドミノを置いていく方がよい = 雪だるま式に状況が良くなる
        let min_op: isize = greedy(&s);
        println!("{}", min_op);
    }
}

fn greedy(sizes: &Vec<usize>) -> isize {
    let n: usize = sizes.len();
    let mut cnt: isize = 1;
    let mut last: usize = 0;
    let mut used: Vec<bool> = vec![false; n];
    used[0] = true;

    loop {
        // Check if current domino can knock down the last one
        if 2 * sizes[last] >= sizes[n - 1] {
            cnt += 1;
            break;
        }

        // Find next domino
        let mut next: isize = -1;
        for i in 1..n {
            if used[i] { continue; }
            if 2 * sizes[last] >= sizes[i] {
                if next != -1 && sizes[next as usize] >= sizes[i] { continue; }
                // sizes[next as usize] < sizes[i]
                next = i as isize;
            }
        }

        // Check if valid next domino found
        if next == -1 || sizes[next as usize] <= sizes[last] {
            return -1;
        }

        last = next as usize;
        used[last] = true;
        cnt += 1;
    }
    return cnt;
}