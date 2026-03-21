use proconio::input;
use std::collections::HashMap;
// ABC439 Divisor(割る数) | Dividend(割られる数) | Quotient(商) | Remainder(余り)
// A. MODに着目してQuotientが同じもの同士を考える．
// A. そして，それらの順序関係と組み合わせの個数を二分探索で計算する．
struct Element {
    index: usize,
    quotient: i64,
}
impl Element {
    fn new(index: usize, quotient: i64) -> Element {
        Element { index, quotient }
    }
}
fn main() {
    input! {n: usize, a: [i64; n]}
    let threes: Vec<Element> = get_elements(3, &a);
    let fives: Vec<Element> = get_elements(5, &a);
    let sevens: Vec<Element> = get_elements(7, &a);
    let three_quot_to_indexes3: HashMap<i64, Vec<usize>> = get_quotient_to_index(&threes);
    let five_quot_to_indexes5: HashMap<i64, Vec<usize>> = get_quotient_to_index(&fives);
    let seven_quot_to_indexes7: HashMap<i64, Vec<usize>> = get_quotient_to_index(&sevens);

    let mut ans: usize = 0;
    for quotient in three_quot_to_indexes3.keys() {
        let mut indexes3: Vec<usize> = three_quot_to_indexes3.get(quotient).unwrap().clone();
        if !five_quot_to_indexes5.contains_key(quotient)
            || !seven_quot_to_indexes7.contains_key(quotient)
        {
            continue;
        }
        let mut indexes5: Vec<usize> = five_quot_to_indexes5.get(quotient).unwrap().clone();
        let mut indexes7: Vec<usize> = seven_quot_to_indexes7.get(quotient).unwrap().clone();
        indexes3.sort();
        indexes5.sort();
        indexes7.sort();

        // 中央の値から決定する．
        for &j in indexes5.iter() {
            let num_left3 = indexes3.partition_point(|&x| x < j);
            let num_right3 = indexes3.len() - num_left3;
            let num_left7 = indexes7.partition_point(|&x| x < j);
            let num_right7 = indexes7.len() - num_left7;

            // Case 1: j is max => i < j and k < j
            ans += num_left3 * num_left7;

            // Case 2: j is min => i > j and k > j
            ans += num_right3 * num_right7;
        }
    }
    println!("{}", ans);
}

fn get_elements(divisor: i64, array: &Vec<i64>) -> Vec<Element> {
    let mut elements: Vec<Element> = Vec::new();
    for i in 0..array.len() {
        if array[i] % divisor == 0 {
            elements.push(Element::new(i, array[i] / divisor));
        }
    }
    return elements;
}

fn get_quotient_to_index(elements: &Vec<Element>) -> HashMap<i64, Vec<usize>> {
    let mut quotient_to_index: HashMap<i64, Vec<usize>> = HashMap::new();
    for i in 0..elements.len() {
        let quotient: i64 = elements[i].quotient;
        let index: usize = elements[i].index;
        let indexes: &mut Vec<usize> = quotient_to_index.entry(quotient).or_insert(Vec::new());
        indexes.push(index);
    }
    return quotient_to_index;
}
