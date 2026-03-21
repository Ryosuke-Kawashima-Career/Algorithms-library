use proconio::{input, marker::Chars};
// abc443E
// Q. Check the reachability of each column
// A. DP: dp[row][col] = (reachable, fully_clear_below)
fn main() {
    input! {t: usize}
    let mut answer: Vec<String> = Vec::new();
    for _case in 0..t {
        // Destinations: (0, i) for i in 0..n
        input! {n: usize, c: usize, s: [Chars; n]}
        let reachable_cleared = solve(c, &s);
        let mut ans: String = String::new();
        for col in 0..n {
            if reachable_cleared[0][col].0 {
                ans.push('1');
            } else {
                ans.push('0');
            }
        }
        answer.push(ans);
    }
    for ans in answer {
        println!("{}", ans);
    }
}

fn solve(c: usize, graph: &Vec<Vec<char>>) -> Vec<Vec<(bool, bool)>> {
    let n: usize = graph.len();
    let mut dp: Vec<Vec<(bool, bool)>> = vec![vec![(false, false); n]; n];
    // Precompute lowest wall for each column
    // lowest_wall[j] = max row index i such that s[i][j] == '#'
    // If no wall, -1.
    let mut lowest_wall: Vec<isize> = vec![-1; n];
    for col in 0..n {
        for row in (0..n).rev() {
            if graph[row][col] == '#' {
                lowest_wall[col] = row as isize;
                break; // Keep the largest row index
            }
        }
    }
    dp[n - 1][c - 1] = (true, true);

    // Distributing DP not Receiving DP
    for row in (1..n).rev() {
        let next_row: usize = row - 1;
        for col in 0..n {
            let (reachable, clear_below) = dp[row][col];
            if !reachable {
                continue;
            }
            for delta_col in -1..=1 {
                let next_col: isize = col as isize + delta_col;
                if next_col < 0 || next_col >= n as isize {
                    continue;
                }
                let next_col: usize = next_col as usize;

                let is_clear_below_target = if next_col == col {
                    // Vertical move: we are at (row, col), so (row, col) is clear.
                    // We need [row..N)[col] to be clear.
                    clear_below
                } else {
                    // Diagonal move: we need [row..N)[col] to be statically clear.
                    lowest_wall[next_col] < row as isize
                };
                if graph[next_row][next_col] == '#' && !is_clear_below_target {
                    continue;
                }
                dp[next_row][next_col].0 = true;
                if is_clear_below_target {
                    dp[next_row][next_col].1 = true;
                }
            }
        }
    }
    return dp;
}
