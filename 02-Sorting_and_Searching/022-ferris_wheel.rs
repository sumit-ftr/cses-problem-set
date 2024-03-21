use std::io::Write;

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n: usize = token.next();
    let x: u32 = token.next();
    let mut v: Vec<u32> = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(token.next());
    }
    v.sort_unstable();

    let (mut l, mut r, mut count) = (0usize, n - 1, 0usize);
    while l <= r {
        if v[l] + v[r] <= x {
            l += 1;
        }
        count += 1;
        if r == 0 {
            break;
        }
        r -= 1;
    }
    writeln!(out, "{count}").unwrap();
}

pub struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitAsciiWhitespace<'static>,
}

impl<R: std::io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}
