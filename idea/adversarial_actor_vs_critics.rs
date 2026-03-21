use proconio::input;
// ABC441C
// Q. Make sure to get the minimum number of target liquids
// A. Greedy + Adversarial Algorithm
/* まず、高橋君は入っている量が多い方からいくつかのカップを選ぶとしてよいです．これは、現時点ではカップどうしを区別するものは液体の量のみであり、この値が大きなカップを選ぶことが最適な戦略であるからです。高橋君が選んだカップの数を M とします。
次に、”高橋君の敵”は、高橋君の選択を見て、高橋君が飲む日本酒の量をできるだけ小さくするように日本酒を入れるカップを決めます。たとえば、入っている量が少ない方から K 個に日本酒を入れればよいです。
高橋君および”高橋君の敵”がこのように行動したときに、高橋君が X ml 以上の日本酒を飲める最小の M が答えになります。*/
fn main() {
    // Actor vs Critics
    input! {n: usize, k: usize, x: i64, mut a: [i64; n]}
    a.sort();
    let mut sum_of_target: i64 = 0;
    // (n - k) means the resistance from the critics
    // rev() means the actor's choices
    for i in (0..k).rev() {
        sum_of_target += a[i];
        if sum_of_target >= x {
            println!("{}", n - i);
            return;
        }
    }
    println!("-1");
}
