use proconio::input;
// abc333e
// 反転クエリ: クエリを逆から見る.(クエリの先読み)
// 貪欲法
fn main() {
    input!{n: usize, queries: [(usize, usize); n]}
    let mut choices: Vec<usize> = Vec::new();
    let mut enemies = std::collections::HashMap::new();
    // 後ろから敵を記録する
    // 必要なポーションをできるだけギリギリのタイミングで拾う
    for &(query, x) in queries.iter().rev() {
        // ポーションを拾うと敵は死ぬ
        if query == 1 {
            if let Some(enemy) = enemies.get_mut(&x) {
                if *enemy == 1 {
                    enemies.remove(&x);
                } else {
                    *enemy -= 1;
                }
                choices.push(1);
            } else {
                choices.push(0);
            }
        } else {
            *enemies.entry(x).or_insert(0) += 1;
        }
    }

    // 敵を倒しきれていないとき、ゲームオーバー
    if !enemies.is_empty() {
        println!("-1");
        return;
    }

    // クエリを順転する。
    choices.reverse();
    let mut max_drink: usize = 0;
    let mut drink_index: usize = 0;
    let mut drink: usize = 0;
    for &(query, _) in queries.iter() {
        if query == 1 {
            // 値が更新されるときに、最大値に挑戦できる。
            if choices[drink_index] == 1 {
                drink += 1;
                max_drink = max_drink.max(drink);
            }
            drink_index += 1;
        } else {
            drink = drink.saturating_sub(1);
        }
    }

    println!("{}", max_drink);
    for &choice in choices.iter() {
        print!("{} ", choice);
    }
}