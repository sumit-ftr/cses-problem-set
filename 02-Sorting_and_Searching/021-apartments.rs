use std::io::Write;

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n = token.next::<usize>();
    let m = token.next::<usize>();
    let k = token.next::<u32>();
    let mut applicants = Vec::<u32>::with_capacity(n);
    let mut apartments = Vec::<u32>::with_capacity(m);
    for _ in 0..n {
        applicants.push(token.next());
    }
    for _ in 0..m {
        apartments.push(token.next());
    }
    applicants.sort_unstable();
    apartments.sort_unstable();

    let (mut reserved, mut i, mut j) = (0u32, 0usize, 0usize);
    while i < n && j < m {
        if applicants[i] as i64 - k as i64 <= apartments[j] as i64
            && apartments[j] <= applicants[i] + k
        {
            i += 1;
            j += 1;
            reserved += 1;
        } else if applicants[i] as i64 - k as i64 > apartments[j] as i64 {
            j += 1;
        } else {
            i += 1;
        }
    }
    writeln!(out, "{reserved}").unwrap();
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
