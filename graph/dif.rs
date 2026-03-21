const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [
    (N1, 0),
    (0, N1), (0, 1),
    (1, 0)
];
const D8: [(usize, usize); 8] = [
    (N1, N1), (N1, 0), (N1, 1),
    (0, N1), (0, 1),
    (1, N1), (1, 0), (1, 1)
];

fn main() {
    let mut graph: Vec<Vec<usize>> = vec![vec![0; w]; h];
    let dif: Vec<(i64, i64)> = vec![
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
        (1, -1), (1, 0), (1, 1)
    ];
    for i in 0..h {
        for j in 0..w {
            if S[i][j] == '#' {
                for &(dy, dx) in dif.iter() {
                    if 0 <= (i as i64 + dy) && (i as i64 + dy) < h as i64
                    && 0 <= (j as i64 + dx) && (j as i64 + dx) < w as i64 {
                        // statement
                        let next_y: usize = i + dy as usize;
                        let next_x: usize = j + dx as usize;
                    }
                }
            }
        }
    }

    for i in 0..4 {
        for j in 0..4 {
            for di in -4..=4 {
                for dj in -4..=4 {
                    if 0 <= i as i64 + di && i as i64 + di < 4
                    && 0 <= j as i64 + dj && j as i64 + dj < 4 {
                        let next_i: usize = (i as i64 + di) as usize;
                        let next_j: usize = (j as i64 + dj) as usize;
                    }
                }
            }
        }
    }

    for &(dy, dx) in D4.iter() {
        let next_y: usize = i + dy;
        let next_x: usize = j + dx;
        let next_y: usize = y.wrapping_add(dy);
        let next_x: usize = x.wrapping_add(dx);
        if next_y < h && next_x < w {
        }
    }
}