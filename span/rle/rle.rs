// [(value, numbers)]
fn rle(s: &Vec<i64>) -> Vec<(i64, usize)> {
    let mut res: Vec<(i64, usize)> = Vec::new();
    let n: usize = s.len();

    let mut i: usize = 0;
    while i < n {
        let mut j: usize = i;
        // jが条件を満たす範囲から1つ分，はみ出す感じ
        while j < n && s[i] == s[j] {
            j += 1;
        }
        // j-i: numbers
        res.push((s[i], j - i));
        i = j;
    }

    return res;
}

fn rle<T: PartialEq + Copy>(array: &[T]) -> Vec<(T, usize)> {
    let mut res: Vec<(T, usize)> = Vec::new();
    let n: usize = array.len();
    let mut l: usize = 0;
    while l < n {
        let mut r: usize = l + 1;
        while r < n && array[r] == array[l] {
            r += 1;
        }
        res.push((array[l], r - l));
        l = r;
    }
    return res;
}
