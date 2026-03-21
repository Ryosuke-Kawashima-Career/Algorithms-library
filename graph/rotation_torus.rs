use proconio::input;
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [
    (N1, 0),
    (0, 1),
    (1, 0),
    (0, N1),
];
// abc339b
// トーラス(円環)状のグリッド
// graph[i][j] = 0: graphを1に変換し時計回り, graph[i][j] = 1: 逆の操作
fn main() {
    input!{h: usize, w: usize, n: usize}
    // 0: . 1: #
    let mut graph: Vec<Vec<usize>> = vec![vec![0; w]; h];
    torus_mod(&mut graph, n);
    print_grid(&graph);
}

fn torus_mod(graph: &mut Vec<Vec<usize>>, repeat: usize) {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    let mut cur_y: usize = 0;
    let mut cur_x: usize = 0;
    // playerの向きを記録する
    let mut cur_dir: usize = 0;

    for _ in 0..repeat {
        if graph[cur_y][cur_x] == 0 {
            cur_dir = (cur_dir + 1) % 4;
        } else {
            // mod4なので -1==3 (mod4) 
            cur_dir = (cur_dir + 4 - 1) % 4;
        }
        let (dy, dx) = D4[cur_dir];
        // torusをmodで表現する
        let next_y: usize = (cur_y + h).wrapping_add(dy) % h;
        let next_x: usize = (cur_x + w).wrapping_add(dx) % w;
        // xorでon-offを表現する
        graph[cur_y][cur_x] ^= 1;
        cur_y = next_y;
        cur_x = next_x;
    }
}

fn torus_paraphrase(graph: &mut Vec<Vec<usize>>, repeat: usize) {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    // -1を [h-1, w-1] で表現する
    // y: mod h, x: mod w
    let d4: [(usize, usize); 4] = [
        (h-1, 0),
        (0, 1),
        (1, 0),
        (0, w-1),
    ];
    let mut cur_dir: usize = 0;
    let mut cur_y: usize = 0;
    let mut cur_x: usize = 0;

    for _ in 0..repeat {
        if graph[cur_y][cur_x] == 0 {
            cur_dir += 1;
            cur_dir %= 4;
        } else {
            // 向きを-1変更することを+3として言い換える
            cur_dir += 3;
            cur_dir %= 4;
        }
        graph[cur_y][cur_x] ^= 1;

        let (dy, dx) = d4[cur_dir];
        // modで循環を表現する
        cur_y += dy;
        cur_y %= h;
        cur_x += dx;
        cur_x %= w;
    }
}

fn print_grid(graph: &Vec<Vec<usize>>) {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    for i in 0..h {
        for j in 0..w {
            if graph[i][j] == 0 {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!("");
    }
}