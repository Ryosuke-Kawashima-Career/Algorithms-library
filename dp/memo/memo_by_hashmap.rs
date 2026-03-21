use proconio::input;
use std::collections::HashMap;
// abc340C
fn main() {
    input!{n: usize}
    let mut map = HashMap::new();
    let ans = dfs(n, &mut map);
    println!("{}", ans);
}

fn ceil_div(x: usize, y: usize) -> usize {
    if x % y == 0 {
        return x / y;
    }
    return x / y + 1;
}

// memo dpの応用
fn dfs(num: usize, map: &mut HashMap<usize, usize>) -> usize {
    if num == 1 {
        map.insert(1, 0);
        return 0;
    }
    let mut res: usize = num;

    let floor: usize = num / 2;
    let ceil: usize = ceil_div(num, 2);
    if let Some(val) = map.get(&floor) {
        res += val;
    } else {
        res += dfs(floor, map);
    }
    if let Some(val) = map.get(&ceil) {
        res += val;
    } else {
        res += dfs(ceil, map);
    }

    // memo and return
    map.insert(num, res);
    return res;
}