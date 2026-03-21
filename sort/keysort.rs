fn main() {
    // 大きい順に並び替える。
    vec.sort_by(|a, b| b.cmp(a));

    // tuple
    tup_vec.sort_by_key(|x| x.0);

    // partial reverse
    numbers[l..=r].reverse();

    // index sort(vector: [(usize, usize)]) 0: index 1: value
    vector.sort_by(|a, b| if a.1 == b.1 {
        (a.0).cmp(&b.0)
    } else {
        (b.1).cmp(&a.1)   
    });

    // f64
    d.sort_by(|a, b| b.partial_cmp(a).unwrap());
    ab.sort_by(|&x, &y| (y.1 as f64 / y.0 as f64).partial_cmp(&(x.1 as f64 / x.0 as f64)).unwrap());

    // struct
    edges.sort_by(|a, b| a.weight.cmp(&b.weight)); // small -> large
    points.sort_by_key(|point| point.x);
    vec.sort_by_key(|w| std::cmp::Reverse(*w));

    // string
    words.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
}