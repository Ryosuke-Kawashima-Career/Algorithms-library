use proconio::input;
// ordered map @beg323
fn main() {
    input!{n: usize}
    let mut map = std::collections::BTreeMap::new();
    for _ in 0..n {
        input!{s: usize, c: usize}
        map.insert(s, c);
    }
    let mut ans: usize = 0;

    while let Some((mut size, mut num)) = map.pop_first() {
        ans += num % 2;
        // ignore when number is 1
        while num > 1 {
            size *= 2;
            num /= 2;
            *map.entry(size).or_insert(0) += num % 2;
        }
    }
    // acc: accumulator
    let mut heads = h.into_iter().fold(BTreeMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });
    // map maxkey
    if let Some((&max_key, &val)) = map.iter().next_back() {
    }
    // map minkey
    if let Some((&min_key, &val)) = map.iter().next() {
    }

    // max value
    // 値が更新されるときに最大に挑む挑戦権が与えられる。
    if updated {
        *map.entry(key).or_insert(0) += 1;
        max_value = max_value.max(map[&key]);
    }
    // vector
    let mut map = BTreeMap::<usize, Vec<usize>>new();
    for i in 0..n {
        map.entry(i).or_default().push(vector[i]);
    }
    // update the map for maximizing its value
    map_max
        .entry(key)
        .and_modify(|v| (*v) = (*v).max(new_value))
        .or_insert(new_value);

    // binary search
    if let Some((&key, _)) = map.range(target..).next() {
        if let Some(num) = map.get_mut(&key) {
            if *num > 1 {
                *num -= 1;
            } else {
                map.remove(&key);
            }
        }
    } else {
        println!("-1");
        return;
    }

    println!("{}", ans);
}