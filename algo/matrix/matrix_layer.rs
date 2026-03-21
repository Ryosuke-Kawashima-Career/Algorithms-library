use proconio::{input, marker::Chars};
// abc375C: rotate by layer
// matrix layer && rotation by Mod4
fn main() {
    input!{n: usize, a: [Chars; n]}
    let cur: Vec<Vec<char>> = rotate_right(&a);
    print_2d(&cur);
}

fn rotate_right(p: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n: usize = p.len();
    let mut res: Vec<Vec<char>> = p.clone();
    for x in 0..n {
        for y in 0..n {
            let layer: usize = x.min(y).min(n-1-x).min(n-1-y);
            let mut cur_x: usize = x;
            let mut cur_y: usize = y;
            for _ in 0..((layer+1) % 4) {
                let next_x: usize = cur_y;
                let next_y: usize = n-1-cur_x;
                cur_x = next_x;
                cur_y = next_y;
            }
            res[cur_x][cur_y] = p[x][y];
        }
    }
    return res;
}

fn print_2d<T: std::fmt::Display>(graph: &Vec<Vec<T>>) {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    for i in 0..h {
        for j in 0..w {
            print!("{}", graph[i][j]);
        }
        println!("");
    }
}