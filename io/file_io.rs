use std::{fs::File, io::{Read, BufReader, Write, BufWriter, Result}};
// 鉄則A50
fn main() {
    // Cargo.tomlがあるディレクトリが基準: ex. "./src/main.rs"
    let vector1 = read_file("./input/input.txt");
    // buf is faster
    let vector2 = bufread_file("./input/input.txt");

    write_file("./myout.txt", &operations);
    // return Result<()>
    bufwrite_file("./bufout.txt", &operations);
}

fn read_file(file_name: &str) -> Vec<Vec<i64>> {
    let mut a: Vec<Vec<i64>> = vec![vec![0; N]; N];
    let mut f: File = File::open(file_name).expect("Fail to open file!");
    let mut contents: String = String::new();
    f.read_to_string(&mut contents).expect("Fail to read file!");
    
    let mut lines = contents.lines();
    let mut cur_line: usize = 0;
    while let Some(line) = lines.next() {
        let mut nums = line.split_whitespace();
        let mut cur_col: usize = 0;
        while let Some(num) = nums.next() {
            let num = num.parse().unwrap();
            a[cur_line][cur_col] = num;
            cur_col += 1;
        }
        cur_line += 1;
    }

    return a;
}
fn bufread_file(file_name: &str) -> Vec<Vec<i64>> {
    let mut a: Vec<Vec<i64>> = vec![vec![0; N]; N];
    let mut f = BufReader::new(File::open(file_name).unwrap());
    let mut contents: String = String::new();
    f.read_to_string(&mut contents).expect("Fail to read file!");
    
    let mut lines = contents.lines();
    let mut cur_line: usize = 0;
    while let Some(line) = lines.next() {
        let mut nums = line.split_whitespace();
        let mut cur_col: usize = 0;
        while let Some(num) = nums.next() {
            let num = num.parse().unwrap();
            a[cur_line][cur_col] = num;
            cur_col += 1;
        }
        cur_line += 1;
    }

    return a;
}

fn write_file(file_name: &str, operations: &Vec<(usize, usize, i64)>) {
    let mut f = File::create(file_name).expect("Fail to create a file!");
    let q: usize = operations.len();
    f.write_all(&format!("{}\n", q).as_bytes()).expect("Fail to write!");

    for &(y, x, h) in operations.iter() {
        f.write_all(&format!("{} {} {}\n", x, y, h).as_bytes()).expect("Fail to write!");
    }
}
fn bufwrite_file(file_name: &str, operations: &Vec<(usize, usize, i64)>) -> Result<()> {
    let mut f = BufWriter::new(File::create(file_name).unwrap());
    let q: usize = operations.len();
    f.write_all(&format!("{}\n", q).as_bytes())?;
    
    for &(y, x, h) in operations.iter() {
        f.write_all(&format!("{} {} {}\n", x, y, h).as_bytes())?;
    }

    Ok(())
}