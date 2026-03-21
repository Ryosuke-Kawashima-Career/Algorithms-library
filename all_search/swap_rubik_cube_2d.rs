use proconio::input;
use itertools::Itertools;
const INF: usize = 1 << 60;
// abc332d
// 数列(1,2,…,N)を、隣接する2項を入れ替える操作を繰り返して所望の順列 A=(A1, A2 ,…,A N)に作り替えるための最小操作回数は
// 順列Aの転倒数、つまり、1≤i<j≤N かつAi>Ajを満たす組 (i,j) の個数
// permutationで全探索
fn main() {
    input!{h: usize, w: usize, a: [[usize; w]; h], b: [[usize; w]; h]}
    let mut ans: usize = INF;

    // ルービックキューブのシャッフル: permutation
    for row_perm in (0..h).permutations(h) {
        for col_perm in (0..w).permutations(w) {
            // 試行回数を転倒数で編集距離を計算する
            let row_inv: usize = count_inversions(&row_perm);
            let col_inv: usize = count_inversions(&col_perm);
            if judge_same(&a, &b, &row_perm, &col_perm) {
                ans = ans.min(row_inv + col_inv);
            }
        }
    }

    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

fn judge_same(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>, 
    row_perm: &Vec<usize>, col_perm: &Vec<usize>
) -> bool {
    let mut all_same = true;
    let h: usize = row_perm.len();
    let w: usize = col_perm.len();

    for i in 0..h {
        for j in 0..w {
            if a[i][j] != b[row_perm[i]][col_perm[j]] {
                all_same = false;
            }
        }
    }

    return all_same;
}

fn count_inversions(perm: &Vec<usize>) -> usize {
    let n: usize = perm.len();
    let mut invs: usize = 0;
    for i in 0..n {
        for j in i+1..n {
            // inversion: i < j && a[i] > a[j]
            if perm[i] > perm[j] {
                invs += 1;
            }
        }
    }

    return invs;
}