use superslice::Ext;
use ordered_float::NotNan;

// lower_bound: x未満の個数を返す  以上のindex   
// upper_bound: x以下の個数を返す  より大きい数字のindex

fn main() {
    let a = [10, 11, 13, 13, 15];
    assert_eq!(a.lower_bound(&10), 0);
    assert_eq!(a.lower_bound(&12), 2);
    let b = [1, 2, 3, 6, 9, 9];
    assert_eq!(b.lower_bound(&3), b.lower_bound_by(|x| x.cmp(&3)));
    let b = [1, 2, 3, 6, 9, 9];
    assert_eq!(b.lower_bound(&3), b.lower_bound_by_key(&6, |x| x * 2));

    let a = [10, 11, 13, 13, 15];
    assert_eq!(a.upper_bound(&10), 1);
    assert_eq!(a.upper_bound(&12), 2);

    // 未満の数: lower, 以下の数: upper
    // b未満の個数
    let smaller: usize = a.lower_bound(&b);
    // bより大きい個数
    let equal_larger: usize = n - c.upper_bound(&b);
    
    // f64
    vec.lower_bound_by(|x| x.partial_cmp(&num).unwrap())
    vec.lower_bound(&NotNan::new(num).unwrap());

    // std < : lowerbound <= : upperbound 
    let vec = vec![1, 2, 3, 3, 5, 6, 7];
    let index = vec.partition_point(|&x| x < 5);
    assert_eq!(index, 4);
}