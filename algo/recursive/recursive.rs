// 同じことを繰り返すときに使う。
// 参照しないベクトルや文字列を引数にすることで途中の計算を記録できる。
fn recursive(n: usize, mut res_vec: Vec<i64>, mut res_string: String, ans: &mut ans) -> i64 {
    // answer
    let mut res: i64 = 0;
    // base case
    if n == 0 {
        return 0;
    }

    // update &mut
    *ans = (*ans).max(res);

    // 行きがけにデータを子に渡す
    // recurrence formula(漸化式の遷移)
    res = recursive(n-1, res_vec.clone(), res_string.clone(), ans);
    // 帰りがけにデータを復元したり，子からデータを受け取る

    // end to compile
    return res;
}
// 多次元配列(abc322d)
fn recursive_2d(grid: &mut Vec<Vec<(i64, i64)>>, cnt: usize) {
    if cnt == 0 {
        return;
    }
    change(&mut grid[cnt-1]);
    recursive_2d(grid, cnt-1);
}

// abd165c
fn main() {
    let mut all_a: Vec<Vec<i64>> = Vec::new();
    calc_all(n, m, Vec::new(), &mut all_a);
    let mut all_vectors: Vec<Vec<i64>> = Vec::new();
    enum_all(n, m, vec![], &mut all_vectors);
}

// n: limit
fn calc_all(n: usize, m: i64, mut a: Vec<i64>, all_a: &mut Vec<Vec<i64>>) {
    let length: usize = a.len();
    if length == n {
        all_a.push(a);
        return;
    }

    let top: i64 = if length == 0 {
        1
    } else {
        a[length-1]
    };
    for num in top..=m {
        a.push(num);
        calc_all(n, m, a.clone(), all_a);
        a.pop();
    }
}

// nは残りの要素数(remain)
fn enum_all(n: usize, m: i64, mut vector: Vec<i64>, all_vectors: &mut Vec<Vec<i64>>) {
    if n == 0 {
        all_vectors.push(vector);
        return;
    }

    let top: i64 = if vector.len() == 0 {
        0
    } else {
        vector[vector.len()-1]
    };

    for val in top..=m {
        vector.push(val);
        enum_all(n-1, m, vector.clone(), all_vectors);
        vector.pop();
    }
}

fn abc326d() {
    let mut rows: Vec<Vec<char>> = Vec::new();
    for perm in (0..n).permutations(3) {
        let mut row: Vec<char> = vec!['.'; n];
        row[perm[0]] = 'A';
        row[perm[1]] = 'B';
        row[perm[2]] = 'C';
        rows.push(row);
    }

    let mut mats: Vec<Vec<Vec<char>>> = Vec::new();
    let mut mat: Vec<Vec<char>> = Vec::new();
    enum_mat(n, &rows, &mut mat, &mut mats);
}

fn enum_mat(n: usize, rows: &Vec<Vec<char>>, mat: &mut Vec<Vec<char>>, mats: &mut Vec<Vec<Vec<char>>>) {
    // base case
    if n == 0 {
        mats.push(mat.clone());
        return;
    }

    for row in rows.iter() {
        // 行きがけ
        mat.push(row.clone());
        enum_mat(n-1, rows, mat, mats);
        mat.pop();
    }
}