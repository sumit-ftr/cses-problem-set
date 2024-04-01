use std::io::Write;

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n = token.next::<usize>();
    let x = token.next::<u32>();
    let mut v = Vec::<u32>::with_capacity(n);
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
                return token.parse().ok().unwrap();
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
