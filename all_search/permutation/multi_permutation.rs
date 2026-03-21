use proconio::input;
// abc416c
// Q. N^K sequences are sorted in lexicographical order, find the X-th smallest string.
// A1. DFS A2. 重複順列
fn main() {
    input!{n: usize, k: usize, x: usize, s: [String; n]}
    let products = multi_permutation(n, k);

    let mut strings: Vec<String> = Vec::new();
    for indexes in products {
        let mut string = String::new();
        for &index in indexes.iter() {
            string = format!("{}{}", string, s[index]);
        }
        strings.push(string);
    }
    strings.sort();
    println!("{}", strings[x-1]);
}

fn multi_permutation(n: usize, k: usize) -> Vec<Vec<usize>> {
    /*
        Args:
            n: selected values [0..n)
            k: number of repetition
        Returns:
            the products of indexes (重複順列)
    */
    let mut products: Vec<Vec<usize>> = vec![vec![]; 0];
    // N^Kの全探索
    for mut bit_n in 0..(n.pow(k as u32)) {
        let mut indexes: Vec<usize> = Vec::new();
        for _ in 0..k {
            let index: usize = bit_n % n;
            indexes.push(index);
            bit_n /= n;
        }
        products.push(indexes);
    }

    products.sort();
    return products;
}

#[allow(dead_code)]
fn enumerate(length: usize, s: &Vec<String>) -> Vec<String> {
    let n: usize = s.len();
    let mut res: Vec<String> = Vec::new();
    if length == 0 {
        res.push(String::new());
        return res;
    }

    let res_prev: Vec<String> = enumerate(length - 1, s);
    for string_prev in res_prev {
        for i in 0..n {
            let string = format!("{}{}", string_prev, s[i]);
            res.push(string);
        }
    }
    return res;
}