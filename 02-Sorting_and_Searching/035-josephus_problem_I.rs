use std::io::Write;

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n = token.next::<usize>();
    if n == 1 {
        writeln!(out, "1").unwrap();
    } else {
        let mut prev = Vec::<u32>::with_capacity(n + 1);
        let mut next = Vec::<u32>::with_capacity(n + 1);
        prev.push(0);
        next.push(0);

        prev.push(n as u32);
        prev.push(1);
        for i in 2..n as u32 {
            prev.push(i);
            next.push(i);
        }
        next.push(n as u32);
        next.push(1);

        let mut i = 2usize;
        while prev[i] != next[i] {
            let j = next[next[i] as usize];
            write!(out, "{} ", i).unwrap();
            next[prev[i] as usize] = next[i];
            prev[next[i] as usize] = prev[i];
            i = j as usize;
        }
        writeln!(out, "{} {}", i, next[i]).unwrap();
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
