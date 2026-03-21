fn main() {
    // keyが存在するかの判別が速い。O(logn)
    let mut binary_set = std::collections::BTreeSet::new();

    if query == 1 {
        binary_set.insert(x);
    } else if query == 2 {
        binary_set.remove(&x);
    } else {
        // x未満で最大のとき set.range(..x).last()
        // x以上のうち最小のもの
        match binary_set.range(x..).next() {
            Some(value) => {
                println!("{}", value);
            },
            None => {
                println!("-1");
            }
        }
        // もしくは
        if let Some(num) = set.range(x..).next() {
            println!("{}", num);
        } else {
            println!("-1");
        }
    }

    let min_value: usize = *binary_set.iter().next().unwrap();

    // get max value
    // next_back or last
    let inf: i64 = match binary_set.range(..=x).next_back() {
        Some(&value) => {
            value
        },
        None => {
            0
        }
    };
    // ある値未満の値を取得する．
    if let Some(small) = set.range(..x).next_back() {
    }

    // get min value
    if let Some(min_val) = set.pop_first() {
    }
    // get max value
    if let Some(max_val) = set.pop_last() {
    }

    // get kth biggest
    match set.iter().nth_back(k) {
        Some(&kth) => kth as i64,
        None => -1,
    }
    // get kth smallest
    match set.iter().nth(k) {
        Some(&kth) => kth as i64,
        None => -1,
    }
}