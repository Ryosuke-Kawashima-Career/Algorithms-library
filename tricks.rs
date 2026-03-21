// set RUST_BACKTRACE=1

fn main() {
    // format output: 000001
    println!("{:0>6}", num);
    
    // immutable borrow, mutable borrow in function
    let value = vector[i].clone();
    vector[i+1] = value;
    // assign multi mutables
    let (mut val1, mut val2) = (num1, num2);

    // negative usize
    let neg: i64 = -4;
    println!("{}", 5usize.wrapping_add(neg as usize));
    // with: #[allow(arithmetic_overflow)]
    println!("{}", 5usize + (neg as usize));

    // switch value (cycle of variables)
    let copy_x = x;
    x = y;
    y = copy_x;

    // for and flag　(外部の変数は定数，内部の変数は変数)
    // testすべてを試して判定するflag
    let mut out_flag = true;
    for test in 0..n {
        // test1つごとにflagを1つ判定する
        let mut flag = true;
        if flag {}
    }

    // flag
    let mut all_satisfied = true;
    let mut any_satisfied = false;
    loop {
        if !satisfied {
            all_satisfied = false;
            break;
        }
        if satisfied {
            any_satisfied = true;
            break;
        }
    }

    for i in 0..n {
        // i: const j: variable
        for j in 0..n {
        }
    }
    // endless loop
    for i in 0.. {

    }

    let mut first = 0;
    let mut second = 0;
    if first < value {
        first = value;
    } else if second < value {
        second = value;
    }

    // rle(ランレングス圧縮)
    while i < n {
        while j < n {
            // process before increment
            j += 1;
        }
        i = j;
    }

    // cur_valを定義し，例外を無くす
    let mut cur_val: usize = 0;
    for i in 0..n {
        func(cur_val, vector[i]);
        cur_val = vector[i];
    }

    // span: [start end)
    for span in 1..=n {
        for start in 0..n {
            let end: usize = start + span;
            if end > n {
                continue;
            }
        }
    }

    // mutable i
    for mut num in 1..=n {
    }

    // define life time
    'a: for i in 0..h {
        for j in 0..w {
            if  {
                break 'a;
            }
        }
    }

    // i64: numbers, usize: 基数
    // 0 <= index < vector.len()の境界条件が実装のカギ
    // i=(境界条件)のように試しに代入する。
    if index >= 1 && value > vector[index-1] {}
    let (start, end) = (start-1, end-1);
    if (index as i64) > 1i64 {}
    if index_condition {
        if value_condition {

        }
    } else {
        if value_condition {

        }
    }
    let value = if condition {
        0
    } else {
        // break, continue etc.
        return;
    };
    // 0..=nにindexを収める: max of min, min of max
    let idx_l: usize = 0.max(idx);
    let idx_r: usize = n.min(idx);
    let new_idx: usize = idx.max(0).min(n);
    // 左右を反転する
    let index_rev0 = n - index0 - 1;
    let index_rev1 = n - index1 + 1;
    // thread 'main' panicked at 'index out of bounds: 
    // the len is n but the index is n', src/main.rs:(行数):(文字数(' 'も含む))

    let vector = vec![0, 1, 2];
    let vector_rev: Vec<_> = vector.iter().rev().collect();
    // 重複をsort, dedupで消す。
    vector.sort();
    vector.dedup();
    let idx: usize = 1;
    let index: usize = index.saturating_sub(2);
    let neg_num: usize = num.wrapping_neg();
    if vector1 == vector2 {
    }
    println!("{}", vector[((idx as i64 - 2).abs() % 3) as usize]);
    println!("{}", vector[(idx.abs_diff(2) % 3)]);
    jag[n*y+x] = vector;
    let y = jag_idx / n;
    let x = jag_idx % n;
    for a in a2d.iter() {
        // a: 1D vector, a2d: Vec<Vec<_>>
    }
    // iterator of structures
    for &Edge{to, cost} in graph[vertex].iter() {
    }
    // char -> numbers
    let mut char_vector: Vec<char> = vec!['J'];
    let mut num_vector: Vec<usize> = vec![0; n];
    for i in 0..n {
        match char_vector[i] {
            'J' => num_vector[i] = 0,
            'O' => num_vector[i] = 1,
            'I' => num_vector[i] = 2,
            _ => println!("Input Error!"),
        }
    }

    // slice &Vec<T> = &[T]
    let mid_idx: usize = n / 2;
    // length of &vector[m..n] = n - m
    let a1: &[usize] = &a[0..mid_idx];
    let a2: &[usize] = &a[mid_idx..n];
    // min and max
    let max = *a[i..i+k].iter().max().unwrap();
    let min = *a[i..i+k].iter().min().unwrap();
    if &vector[(vector.len()-3)..vector.len()] == &['A', 'B', 'C'] {}
    // reverse part of vector [l r)
    vector[l..r].reverse();

    // iterator
    let num_of_negative: usize = A.iter().filter(|&&x| x < 0).count();
    let ans: usize = *lengths.iter().max().unwrap();
    let min_num_abs: i64 = A.iter().map(|&x| x.abs()).min().unwrap();
    let sum: i64 = A.iter().map(|&x| x.abs()).sum::<i64>();
    let product: i64 = A.iter().map(|&x| x.abs()).product::<i64>();
    let cloned_vec: Vec<i64> = A.iter().cloned().collect();
    // Vec<(,)> -> Vec<>
    let mut x: Vec<i64> = vector.iter().map(|&(_, x)| x).collect();
    if vector.iter().all(|&y| y >= x) {}
    for (i, perm) in (1..=n).permutations(n).enumerate() {
    }

    // end main func
    match something {
        Some(val) => println!("{}", val),
        None => {
            // rend func
            return;
        }
    }
    // mutable closure
    let mut func_mut = |i: usize| {
        vector[i] += 1;
    };

    // avoid borrow checker by clone()
    for &score in scores[k-1].clone().iter() {
        for i in 0..n {
            scores[k].push(something);
        }
    }

    return;
}

fn arg_tuple((w0, h0): (usize, usize)) {}

fn recursive(vector: Vec<Vec<bool>>, res: String) {
    // inherent properties from parent functions
    recursive(vector.clone(), res.clone());
}