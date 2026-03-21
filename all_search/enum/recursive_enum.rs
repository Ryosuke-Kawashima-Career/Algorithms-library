use proconio::input;
// abc367c: 再帰関数による全列挙
// Q. 以下の条件を満たす長さ Nの整数列を辞書順で小さい方から順に全て出力
// 1. i番目の要素は1以上 2. Ri以下総和がKの倍数
// A. 方針1. 再帰を使って列挙する
// 1. 最初に1つ目の要素を小さい順に固定する
// 2. 1つ目の要素を固定した下に再帰を呼び、 
// 3. 2つ目の要素を小さい順に固定する
// 4. 要素が出そろったら、総和がKの倍数であるかを判定した後出力する
fn main() {
    input!{n: usize, k: usize, r: [usize; n]}
    let mut answer: Vec<usize> = vec![0; n];
    solve(0, 0, k, &r, &mut answer);
}

fn print_vector(x: &Vec<usize>) {
    let n: usize = x.len();
    for i in 0..n {
        print!("{} ", x[i]);
    }
    println!("");
}

fn solve(index: usize, sum_mod: usize, k: usize, 
    r: &Vec<usize>, answer: &mut Vec<usize>) 
{
    let n: usize = r.len();
    if index == n {
        if sum_mod % k == 0 {
            print_vector(answer);
        }
        return;
    } 
    for num in 1..=r[index] {
        answer[index] = num;
        solve(index+1, (sum_mod + num) % k, k, r, answer);
    }
}