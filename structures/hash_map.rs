fn main() {
    let mut map = std::collections::HashMap::new();
    // count key's value
    *map.entry(key).or_insert(0) += 1;
    map.get(key); // Some(&value)
    // HashMap<usize, Vec<usize>>
    (*map.entry(key).or_insert(vec![])).push(one_value);

    let (&key, &val) = map.iter().next().unwrap();

    input!{query: usize}
    if query == 1 {
        input!{x: String, y: usize}
        // key, value
        // すでに値がある場合は上書きされる．
        map.insert(x, y);
    } else {
        input!{x: String}
        let value: usize = map[&x];
        // map.get(&x).unwrap()
        println!("{}", value);
    }

    // for
    for (&key, &val) in map.iter() {
    }
    // if key is a vector -> key.clone()
    for (key, &val) in map.iter() {
    }
    // count
    *map.entry(key).or_insert(0) += 1;
    // min value
    let entry = map.entry(key).or_insert(value);
    (*entry) = (*entry).min(value);
    // vector
    let mut map = BTreeMap::<usize, Vec<usize>>new();
    for i in 0..n {
        map.entry(i).or_default().push(vector[i]);
    }

    // max value
    // 値が更新されるときに最大に挑む挑戦権が与えられる。
    if updated {
        *map.entry(key).or_insert(0) += 1;
        max_value = max_value.max(map[&key]);
    }
    // update the map for maximizing its value
    map_max
        .entry(key)
        .and_modify(|v| (*v) = (*v).max(new_value))
        .or_insert(new_value);

    // keyの値をあらかじめコピー(Some(&x)etc.)するか, Clone(key.clone())する
    if let Some(x) = used_nums.get_mut(&key) {
        if *x == 1 {
            used_nums.remove(&key);
        } else {
            *x -= 1;
        }
    }
}