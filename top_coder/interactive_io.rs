use std::io::Write;
// abc244C
fn main() {
    input!{new_stdin_parser = parser, }
    input!{parser=parser, n: usize}
    std::io::stdout().flush().unwrap();

    let mut rem: std::collections::BTreeSet<_> = (1..=2 * n + 1).collect();
    for _ in 0..=n {
        let x = *rem.iter().next().unwrap();
        rem.remove(&x);
        let reply = query(x, &mut parser);
        rem.remove(&reply);
    }
}

fn query<R: std::io::BufRead>(x: usize, parser: &mut Parser<R>) -> usize {
    println!("{}", x);
    input!{parser=parser, y: usize}
    std::io::stdout().flush().unwrap();
    y
}

#[macro_export]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut parser = Parser::from_str($s);
        input_inner!{parser, $($r)*}
    };
    (parser = $parser:ident, $($r:tt)*) => {
        input_inner!{$parser, $($r)*}
    };
    (new_stdin_parser = $parser:ident, $($r:tt)*) => {
        let stdin = std::io::stdin();
        let reader = std::io::BufReader::new(stdin.lock());
        let mut $parser = Parser::new(reader);
        input_inner!{$parser, $($r)*}
    };
    ($($r:tt)*) => {
        input!{new_stdin_parser = parser, $($r)*}
    };
}

#[macro_export]
macro_rules! input_inner {
    ($parser:ident) => {};
    ($parser:ident, ) => {};
    ($parser:ident, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($parser, $t);
        input_inner!{$parser $($r)*}
    };
    ($parser:ident,mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($parser, $t);
        input_inner!{$parser $($r)*}
    };
}

#[macro_export]
macro_rules! read_value {
    ($parser:ident, ( $($t:tt),* )) => {
        ( $(read_value!($parser, $t)),* )
    };
    ($parser:ident, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($parser, $t)).collect::<Vec<_>>()
    };
    ($parser:ident, [ $t:tt ]) => {{
        let len = read_value!($parser, usize);
        (0..len).map(|_| read_value!($parser, $t)).collect::<Vec<_>>()
    }};
    ($parser:ident, Chars) => {
        read_value!($parser, String).chars().collect::<Vec<char>>()
    };
    ($parser:ident, digits) => {
        read_value!($parser, String).chars().map(|c| c as i64 - '0' as i64).collect::<Vec<i64>>()
    };
    ($parser:ident, char_) => {
        read_value!($parser, String).chars().collect::<Vec<char>>()[0]
    };
    ($parser:ident, Usize1) => {
        read_value!($parser, usize) - 1
    };
    ($parser:ident, line) => {
        $parser.next_line()
    };
    ($parser:ident, line_) => {
        $parser.next_line().chars().collect::<Vec<char>>()
    };
    ($parser:ident, $t:ty) => {
        $parser.next::<$t>().expect("Parse error")
    };
}

pub struct Parser<R> {
    pub reader: R,
    buf: std::collections::VecDeque<u8>,
    parse_buf: Vec<u8>,
}

impl Parser<std::io::Empty> {
    pub fn from_str(s: &str) -> Parser<std::io::Empty> {
        Parser {
            reader: std::io::empty(),
            buf: std::collections::VecDeque::from(s.as_bytes().to_vec()),
            parse_buf: vec![],
        }
    }
}

impl<R: std::io::BufRead> Parser<R> {
    pub fn new(reader: R) -> Parser<R> {
        Parser {
            reader: reader,
            buf: std::collections::VecDeque::new(),
            parse_buf: vec![],
        }
    }
    pub fn update_buf(&mut self) {
        loop {
            let (len, complete) = {
                let buf2 = self.reader.fill_buf().unwrap();
                self.buf.extend(buf2.iter());
                let len = buf2.len();
                (len, buf2.last() < Some(&0x20))
            };
            self.reader.consume(len);
            if complete {
                break;
            }
        }
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> Result<T, T::Err> {
        loop {
            while let Some(c) = self.buf.pop_front() {
                if c > 0x20 {
                    self.buf.push_front(c);
                    break;
                }
            }
            self.parse_buf.clear();
            while let Some(c) = self.buf.pop_front() {
                if c <= 0x20 {
                    self.buf.push_front(c);
                    break;
                } else {
                    self.parse_buf.push(c);
                }
            }
            if self.parse_buf.is_empty() {
                self.update_buf();
            } else {
                return unsafe { std::str::from_utf8_unchecked(&self.parse_buf) }.parse::<T>();
            }
        }
    }
    pub fn next_line(&mut self) -> String {
        loop {
            while let Some(c) = self.buf.pop_front() {
                if c >= 0x20 {
                    self.buf.push_front(c);
                    break;
                }
            }
            self.parse_buf.clear();
            while let Some(c) = self.buf.pop_front() {
                if c < 0x20 {
                    self.buf.push_front(c);
                    break;
                } else {
                    self.parse_buf.push(c);
                }
            }
            if self.parse_buf.is_empty() {
                self.update_buf();
            } else {
                return unsafe { std::str::from_utf8_unchecked(&self.parse_buf) }.to_string();
            }
        }
    }
}
