use proconio::input;
// abc382d
// Q. 1≤Ai, 2以上N以下の各整数iに対してAi−1+10≤Ai, AN≤Mを辞書順に列挙
// A. Recursive
fn main() {
    input!{n: usize, m: i64}
    let mut answer: Vec<Vec<i64>> = Vec::new();
    enum_lexico(n, m, vec![], &mut answer);
    println!("{}", answer.len());
    for ans in answer.iter() {
        for i in 0..n {
            if i == 0 {
                print!("{}", ans[i]);
            } else {
                print!(" {}", ans[i]);
            }
        }
        println!("");
    }
}

fn enum_lexico(remain: usize, limit: i64, mut vec: Vec<i64>, answer: &mut Vec<Vec<i64>>) {
    if remain == 0 {
        answer.push(vec);
        return;
    }
    let lower = if vec.len() == 0 {
        -9
    } else {
        vec[vec.len()-1]
    };
    let upper = limit - 10 * (remain as i64 - 1);
    for ai in (1.max(lower+10))..=upper {
        vec.push(ai);
        enum_lexico(remain-1, limit, vec.clone(), answer);
        vec.pop();
    }
}