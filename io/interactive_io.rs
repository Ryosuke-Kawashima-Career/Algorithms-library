use std::fmt::Display;
use std::io::{self, BufRead, Write};

struct InteractiveIO<R: BufRead, W: Write> {
    reader: R,
    writer: W,
}
impl<R: BufRead, W: Write> InteractiveIO<R, W> {
    fn new(reader: R, writer: W) -> Self {
        Self { reader, writer }
    }
    fn read_int<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.read_string().parse().unwrap()
    }
    fn read_string(&mut self) -> String {
        let mut buf = String::new();
        if self.reader.read_line(&mut buf).unwrap() == 0 {
            std::process::exit(0);
        }
        buf.trim().to_string()
    }
    // Every protocol message must reach the judge before reading its reply.
    fn write_value(&mut self, value: impl Display) {
        writeln!(self.writer, "{}", value).unwrap();
        self.writer.flush().unwrap();
    }

    fn query(&mut self, index: usize) -> i64 {
        self.write_value(format_args!("? {}", index));
        let value = self.read_int();
        if value == -1 {
            std::process::exit(0);
        }
        value
    }

    fn answer(&mut self, value: i64) {
        self.write_value(format_args!("! {}", value));
    }
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut io = InteractiveIO::new(stdin.lock(), io::BufWriter::new(stdout.lock()));

    let t: usize = io.read_int();
    for _ in 0..t {
        let n: usize = io.read_int();
        io.query(0);
        io.answer(0);
    }
}
