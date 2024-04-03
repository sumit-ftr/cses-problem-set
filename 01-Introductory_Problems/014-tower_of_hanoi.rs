use std::io::{BufWriter, StdoutLock, Write};

fn tower_of_hanoi(n: u8, start: u8, end: u8, out: &mut BufWriter<StdoutLock<'_>>) {
    if n == 1 {
        writeln!(out, "{start} {end}").unwrap();
        return;
    }

    let other: u8 = 6 - (start + end);
    tower_of_hanoi(n - 1, start, other, out);
    writeln!(out, "{start} {end}").unwrap();
    tower_of_hanoi(n - 1, other, end, out);
}

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n = token.next::<u8>();
    let moves = u32::pow(2, n as u32) - 1;
    writeln!(out, "{moves}").unwrap();
    tower_of_hanoi(n, 1, 3, &mut out);
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
