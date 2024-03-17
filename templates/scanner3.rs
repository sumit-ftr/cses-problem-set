use std::io::Write;

fn main() {
    let mut sc = Scanner::new(std::io::stdin().lock());
    let mut op = std::io::BufWriter::new(std::io::stdout().lock());
}

struct Scanner<R> {
    reader: R,
    line: Vec<u8>,
    ptr: usize,
}

#[allow(dead_code)]
impl<R: std::io::BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            line: vec![],
            ptr: 0,
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            while self.ptr < self.line.len() && self.line[self.ptr].is_ascii_whitespace() {
                self.ptr += 1;
            }
            if self.ptr != self.line.len() {
                let start = self.ptr;
                while self.ptr < self.line.len() && !self.line[self.ptr].is_ascii_whitespace() {
                    self.ptr += 1;
                }
                return std::str::from_utf8(&self.line[start..self.ptr])
                    .unwrap()
                    .parse()
                    .ok()
                    .expect("parse error");
            }
            self.line.clear();
            self.reader
                .read_until(b'\n', &mut self.line)
                .expect("read error");
            self.ptr = 0;
        }
    }

    fn line(&mut self) -> Vec<u8> {
        if self.ptr == self.line.len() {
            self.line.clear();
            self.reader
                .read_until(b'\n', &mut self.line)
                .expect("read error");
            self.ptr = 0;
        }
        let result = self.line[self.ptr..].to_vec();
        self.ptr = self.line.len();
        return result;
    }

    fn eof(&mut self) -> bool {
        loop {
            while self.ptr < self.line.len() && self.line[self.ptr].is_ascii_whitespace() {
                self.ptr += 1;
            }
            if self.ptr != self.line.len() {
                return false;
            }
            self.line.clear();
            self.ptr = 0;
            if let Ok(0) = self.reader.read_until(b'\n', &mut self.line) {
                return true;
            }
        }
    }
}
