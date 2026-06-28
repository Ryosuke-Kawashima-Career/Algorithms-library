use std::io::{stdout, BufWriter, Write};
fn main() {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    let ans: usize = 0;
    writeln!(out, "{}", ans).unwrap();
}
