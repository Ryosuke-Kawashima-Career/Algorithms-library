use proconio::input;
// abc347C
// 1~a日目が休日, a+1日目からa+b日目が平日
// dが現在から何日後に予定があるかを表す.
// 一週間a+b日において予定がすべて休日か?
fn main() {
    input!{n: usize, a: usize, b: usize, d: [usize; n]}
    let week = a + b;
    // 二週分計算することで円環を表す
    let mut mods: Vec<usize> = vec![week; n*2];
    for i in 0..n {
        mods[i] = d[i] % week;
        mods[i+n] = mods[i] + week;
    }
    mods.sort();

    let mut is_ok = false;
    // 0日目をfor文で求める
    for i in 0..n {
        // 休日の範囲aに入る日の数をlowerboundで求める
        let holidays: usize = mods.partition_point(|&x| x < mods[i] + a);
        // 休日がn個あるとき，すべてのdが休日になる
        if holidays == i + n {
            is_ok = true;
        }
    }

    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}