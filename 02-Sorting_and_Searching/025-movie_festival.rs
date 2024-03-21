use std::io::Write;

struct Pair {
    s: u32,
    e: u32,
}

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n: usize = token.next();
    let mut v: Vec<Pair> = Vec::with_capacity(n);
    for _ in 0..n {
        let p: Pair = Pair {
            s: token.next(),
            e: token.next(),
        };
        v.push(p);
    }
    v.sort_by(|a, b| (a.e).cmp(&b.e).then((a.s).cmp(&b.s)));
    let mut lmet: u32 = 0; // last movie ending time
    let mut count: u32 = 0;
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
