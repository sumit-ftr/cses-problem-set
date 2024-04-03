use std::io::{BufWriter, StdoutLock, Write};

struct Pair {
    ch: u8,
    count: u8,
}

fn permute(v: &mut [Pair], s: &mut String, n: usize, out: &mut BufWriter<StdoutLock<'_>>) {
    if n == 0 {
        writeln!(out, "{s}").unwrap();
        return;
    }

    for i in 0..v.len() {
        if v[i].count != 0 {
            s.push(v[i].ch as char);
            v[i].count -= 1;
            permute(v, s, n - 1, out);
            s.pop();
            v[i].count += 1;
        }
    }
}

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let s = token.next::<String>();
    let mut v = Vec::<Pair>::new();
    let n = s.len();
    let mut c = 1u128;

    // this step eliminates sorting
    let mut h = vec![0u8; 128];
    for (i, ch) in s.chars().enumerate() {
        c *= i as u128 + 1;
        h[ch as usize] += 1;
    }

    // creating a shorter vector for shorter loop time
    for i in 0..128 {
        if h[i] != 0 {
            v.push(Pair {
                ch: i as u8,
                count: h[i] as u8,
            });
            for i in 2..=h[i] {
                c /= i as u128;
            }
        }
    }
    drop(h);

    writeln!(out, "{c}").unwrap();
    permute(&mut v, &mut "".to_string(), n, &mut out);
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
