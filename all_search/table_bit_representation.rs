use proconio::input;
// ABC407D
// Q. ドミノを長方形に置く
// A. まずは全探索を行う。=> Bit 全探索
// A. horizontal or Vertical=> 2 ^ (h * w)
// Impl. ドミノの置き方を二進数の桁で表現する= 1: domino exists, 0: domino does not exist
// c.f. Profile Dynamic Programming (also known as Broken Profile DP) approach.
fn main() {
    input! {h: usize, w: usize, a: [[usize; w]; h]}
    // Each number = the configuration of horizontal and vertical dominoes
    let domino_configurations: Vec<usize> = enum_configurations(h, w);

    let ans = max_xor(&domino_configurations, &a);
    println!("{}", ans);
}

fn enum_configurations(h: usize, w: usize) -> Vec<usize> {
    let horizontal: usize = (1 << 1) + 1; // Binary 00...11
    let vertical: usize = (1 << w) + 1; // Binary 10...01

    // 0: Empty grid
    let mut configurations: Vec<usize> = vec![0];

    // Iterate through every cell naturally (row by row, col by col)
    let mut cell = 0;
    for i in 0..h {
        for j in 0..w {
            // "tmp" collects new configurations generated at this step
            let mut new_configs: Vec<usize> = Vec::new();

            for &configuration in configurations.iter() {
                // Try placing horizontal domino
                // Check bounds (j+1 < w) AND check if cells are empty ((b & mask) == 0)
                if j + 1 < w && (configuration & (horizontal << cell)) == 0 {
                    new_configs.push(configuration | (horizontal << cell));
                }

                // Try placing vertical domino
                // Check bounds (i+1 < h) AND check if cells are empty
                if i + 1 < h && (configuration & (vertical << cell)) == 0 {
                    new_configs.push(configuration | (vertical << cell));
                }
            }

            // C++: ranges::move(tmp, back_inserter(possible_domino));
            // Rust: Extend the existing list with the new ones.
            // We do NOT remove the old 'b'; keeping 'b' implies "place nothing at this cell".
            configurations.extend(new_configs);

            cell += 1;
        }
    }
    configurations
}

fn max_xor(domino_configurations: &Vec<usize>, a: &Vec<Vec<usize>>) -> usize {
    let h: usize = a.len();
    let w: usize = a[0].len();
    let mut ans: usize = 0;
    for &configuration in domino_configurations.iter() {
        let mut xor: usize = 0;
        for i in 0..h {
            for j in 0..w {
                let cell = i * w + j;
                if (configuration >> cell) & 1 == 0 {
                    xor ^= a[i][j];
                }
            }
        }
        ans = ans.max(xor);
    }
    ans
}
