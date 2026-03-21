use proconio::input;
// abc333e
// 貪欲法: 最初ポーションをできるだけ拾う
// 使わなかったポーションを拾わなかったことにする。(キャンセルカルチャー)
// stackを使ってできるだけギリギリのタイミングでポーションを使う
fn main() {
    // クエリを先読みする
    input!{n: usize, queries: [(usize, usize); n]}
    // (index, type)
    // potions[i: type][j: index](情報が二つあるので二次元配列で管理!!!)
    let mut potions: Vec<Vec<usize>> = vec![vec![]; n+1];
    let mut choices: Vec<u32> = Vec::new();

    // クエリを順転する
    let mut cur_index: usize = 0;
    for &(query, x) in queries.iter() {
        if query == 1 {
            // 貪欲法: とりあえずポーションを拾う
            potions[x].push(cur_index);
            cur_index += 1;
            choices.push(1);
        } else {
            if let Some(index) = potions[x].pop() {
                continue;
            } else {
                // 敵を倒すポーションがないとき
                println!("-1");
                return;
            }
        }
    }

    // 使っていない残りのポーションを拾わなかったことにする(キャンセルカルチャー)
    for x in 1..=n {
        for &index in potions[x].iter() {
            choices[index] = 0;
        }
    }

    let mut max_drink: usize = 0;
    let mut drink: usize = 0;
    cur_index = 0;
    for &(query, x) in queries.iter() {
        if query == 1 {
            // ポーションを拾ったとき
            if choices[cur_index] == 1 {
                drink += 1;
                max_drink = max_drink.max(drink);
            }
            cur_index += 1;
        } else {
            // 敵に対してポーションを使ったとき
            drink -= 1;
        }
    }

    println!("{}", max_drink);
    println!("{}", choices.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(" "));
}