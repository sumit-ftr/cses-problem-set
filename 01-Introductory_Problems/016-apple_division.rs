use std::io::Write;

fn divide(v: &[u32], i: usize, s1: u64, s2: u64, mid: u64) -> u64 {
    if i == v.len() {
        if s1 > s2 {
            return s1 - s2;
        } else {
            return s2 - s1;
        }
    }

    let mut d1 = u64::MAX;
    let mut d2 = u64::MAX;
    if s1 < mid {
        d1 = u64::min(d1, divide(v, i + 1, s1 + v[i] as u64, s2, mid));
    }
    if s2 < mid {
        d2 = u64::min(d2, divide(v, i + 1, s1, s2 + v[i] as u64, mid));
    }
    return u64::min(d1, d2);
}

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n = token.next::<usize>();
    let mut v = Vec::<u32>::with_capacity(n);
    let mut sum = 0u64;
    for i in 0..n {
        v.push(token.next());
        sum += v[i] as u64;
    }

    if n == 1 {
        writeln!(out, "{}", v[0]).unwrap();
    } else {
        writeln!(out, "{}", divide(&v, 0, 0, 0, (sum + 1) / 2)).unwrap();
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
