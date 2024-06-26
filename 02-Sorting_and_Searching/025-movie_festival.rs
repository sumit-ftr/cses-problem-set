use std::io::Write;

struct Pair {
    s: u32,
    e: u32,
}

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n = token.next::<usize>();
    let mut v = Vec::<Pair>::with_capacity(n);
    for _ in 0..n {
        v.push(Pair {
            s: token.next(),
            e: token.next(),
        });
    }
    v.sort_by(|a, b| (a.e).cmp(&b.e).then((a.s).cmp(&b.s)));
    let mut lmet = 0u32; // last movie ending time
    let mut count = 0u32;
    for i in 0..n {
        if lmet <= v[i].s {
            lmet = v[i].e;
            count += 1;
        }
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
