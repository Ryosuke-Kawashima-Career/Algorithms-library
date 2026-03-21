// next permutation
pub fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
    let Some(i) = a.windows(2).rposition(|w| w[0] < w[1]) else { return false };
    let j = a.iter().rposition(|x| x > &a[i]).unwrap();
    a.swap(i, j);
    a[i + 1..].reverse();
    true
}
// no let-else
pub fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
    if let Some(i) = a.windows(2).rposition(|w| w[0] < w[1]) {
        let j = a.iter().rposition(|x| x > &a[i]).unwrap();
        a.swap(i, j);
        a[i + 1..].reverse();
    } else { return false };
    true
}

// shuffle: 長さNの数列Aの置換であって、前K項、後ろN-K項がそれぞれソートされているものを(K,N-K)-shuffle
pub fn next_shuffle<T: Ord>(a: &mut [T], k: usize) -> bool {
    let n = a.len();
    if n == k {
        return false;
    }
    let (left, right) = a.split_at_mut(k);
    let Some(mut i) = left.iter().rposition(|x| x < right.last().unwrap()) else {
        return false;
    };
    let mut j = right.iter().position(|x| &left[i] < x).unwrap();
    std::mem::swap(&mut left[i], &mut right[j]);
    i += 1;
    j += 1;
    let swap_len = (k - i).min(n - k - j);
    left[k - swap_len..].swap_with_slice(&mut right[j..j + swap_len]);
    left[i..].rotate_left(k.saturating_sub(i + swap_len));
    right[j..].rotate_right((n - k).saturating_sub(j + swap_len));
    true
}

// no let-else
pub fn next_shuffle<T: Ord>(a: &mut [T], k: usize) -> bool {
    let n = a.len();
    if n == k {
        return false;
    }
    let (left, right) = a.split_at_mut(k);
    if let Some(mut i) = left.iter().rposition(|x| x < right.last().unwrap()) {
        let mut j = right.iter().position(|x| &left[i] < x).unwrap();
        std::mem::swap(&mut left[i], &mut right[j]);
        i += 1;
        j += 1;
        let swap_len = (k - i).min(n - k - j);
        left[k - swap_len..].swap_with_slice(&mut right[j..j + swap_len]);
        left[i..].rotate_left(k.saturating_sub(i + swap_len));
        right[j..].rotate_right((n - k).saturating_sub(j + swap_len));
    } else {return false; };
    true
}

// ascending shuffle1,…,2Nの置換Aが(2,2,...,2)-shuffle
// A2i-1≤A2i (1≤i≤N)を満たしascending,すなわちA1≤A3≤A5≤⋯≤A2N−1

fn next_pairing(p: &mut [usize]) -> bool {
    let n = p.len();
    let mut used = 0_u32;
    for i in (0..n).rev() {
        used |= 1 << p[i];
        if i % 2 == 1 && p[i] + 1 < used.ilog2() as usize {
            p[i] = (used >> (p[i] + 1)).trailing_zeros() as usize + p[i] + 1;
            used ^= 1 << p[i];
            for i in i + 1..n {
                p[i] = used.trailing_zeros() as usize;
                used ^= 1 << p[i];
            }
            return true;
        }
    }
    false
}

// 自然数の分割: 与えられた正整数 n を正整数の和として表す方法
// ex. 4 = 4, 3 + 1, 2 + 2, 2 + 1 + 1, 1 + 1 + 1 + 1
// Aが自然数Nの分割であるとして、Aよりも辞書順で大きな辞書順最小のNの分割を求める
pub fn next_partition(a: &mut Vec<usize>) -> bool {
    let Some(mut sum) = a.pop() else { return false };
    if a.is_empty() {
        return false;
    }
    while let Some(x) = a.pop() {
        sum += x;
        if a.last().map_or(true, |&last| last > x) {
            a.push(x + 1);
            a.extend(std::iter::repeat(1).take(sum - x - 1));
            break;
        }
    }
    true
}
pub fn prev_partition(a: &mut Vec<usize>) -> bool {
    let Some(i) = a.iter().rposition(|&x| x != 1) else { return false };
    let max = a[i] - 1;
    let mut sum = a.split_off(i).into_iter().sum::<usize>();
    while sum >= max {
        a.push(max);
        sum -= max;
    }
    if sum > 0 {
        a.push(sum);
    }
    true
}

// no let-else
pub fn next_partition(a: &mut Vec<usize>) -> bool {
    if let Some(mut sum) = a.pop() {
        if a.is_empty() {
            return false;
        }
        while let Some(x) = a.pop() {
            sum += x;
            if a.last().map_or(true, |&last| last > x) {
                a.push(x + 1);
                a.extend(std::iter::repeat(1).take(sum - x - 1));
                break;
            }
        }
    } else { return false };
    true
}
pub fn prev_partition(a: &mut Vec<usize>) -> bool {
    if let Some(i) = a.iter().rposition(|&x| x != 1) {
        let max = a[i] - 1;
        let mut sum = a.split_off(i).into_iter().sum::<usize>();
        while sum >= max {
            a.push(max);
            sum -= max;
        }
        if sum > 0 {
            a.push(sum);
        }
    } else { return false };
    true
}

// カルテシアン冪(k?)
let mut a = vec![0; n];
loop {
    match a.iter().rposition(|&x| x + 1 != k) {
        None => break,
        Some(i) => {
            a[i] += 1;
            a[i + 1..].iter_mut().for_each(|x| *x = 0);
        }
    }
}

// next valid paren
// i: ())
// ())))))))))))))()()()()()()()()
// )()))))))))))))()()()()()()()()
// )((((((((()))))))))))))))))))))
match s.windows(3).rposition(|v| v == &['(', ')', ')']) {
    None => break,
    Some(i) => {
        s.swap(i, i + 1);
        s[i + 2..].sort();
    }
};
// or
loop {
    match s.rchunks(2).position(|v| v[0] == ')') {
        None => break,
        Some(count) => {
            let i = s[..n - 2 * count].iter().rposition(|&c| c == '(').unwrap();
            s.swap(i, i + 1);
            s[i + 2..i + 2 + count].iter_mut().for_each(|x| *x = '(');
            s[i + 2 + count..].iter_mut().for_each(|x| *x = ')');
        }
    }
}