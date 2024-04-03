use std::io::Write;

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let t: u32 = token.next();
    for _ in 0..t {
        let x = token.next::<u128>();
        let y = token.next::<u128>();
        let result: u128;
        let max = u128::max(x, y);
        // (max-1)^2 .. max^2
        if max & 1 == 0 {
            // starts from upside
            if x > y {
                result = u128::pow(max, 2) - (y - 1);
            } else {
                result = u128::pow(max - 1, 2) + x;
            }
        } else {
            // starts from leftside
            if x > y {
                result = u128::pow(max - 1, 2) + y;
            } else {
                result = u128::pow(max, 2) - (x - 1);
            }
        }
        writeln!(out, "{result}").unwrap();
    }
}

pub struct Scanner<R> {
    reader: R,
    buffer: Vec<u8>,
    iter: std::str::SplitAsciiWhitespace<'static>,
}

impl<R: std::io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: vec![],
            iter: "".split_ascii_whitespace(),
        }
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.iter.next() {
                return unsafe { token.parse().unwrap_unchecked() };
            }
            self.buffer.clear();
            self.reader.read_until(b'\n', &mut self.buffer).unwrap();
            self.iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buffer);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}
