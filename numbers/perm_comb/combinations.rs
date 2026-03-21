use itertools::Itertools;

fn main() {
    // nPr (n = 3, r = 2)
    println!("Permutation.");
    // vector[perm[i]] : ok
    // len(perm) = r of nPr = permutations(r)
    for perm in (0..3).permutations(2) {
        println!("{:?}", perm);
    }

    // nCr (n = 3, r = 2)
    println!("\nCombination.");
    for comb in (0..3).combinations(2) {
        println!("{:?}", comb);
    }
    // forで実現
    for i in 0..n {
        for j in (i+1)..n {
        }
    }

    // n種類から重複ありでr個(1≦a1≦a2≦…≦ak≦nの選び方): n+r-1Cr
    // n個の玉とr-1の個の仕切りを並び替えることに等しい
    // nHr (n = 3, r = 2)
    println!("\nCombination with replacement.");
    for comb in (0..3).combinations_with_replacement(2) {
        println!("{:?}", comb);
    }
    // for
    for i in 0..n {
        for j in i..n {    
        }
    }
}