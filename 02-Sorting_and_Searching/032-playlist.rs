use std::collections::HashSet;
use std::io::Write;

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n = token.next::<usize>();
    let mut set = HashSet::<u32>::with_capacity(n);
    let mut v = Vec::<u32>::with_capacity(n);
    let (mut start, mut maxcnt) = (0u32, 0u32);
    for i in 0..n as u32 {
        let num = token.next::<u32>();
        v.push(num);
        if let None = set.get(&num) {
            set.insert(num);
            if maxcnt < i - start + 1 {
                maxcnt = i - start + 1;
            }
        } else {
            let mut j = start as usize;
            while v[j] != num {
                set.remove(&v[j]);
                j += 1;
            }
            start += j as u32 + 1 - start;
        }
    }
    writeln!(out, "{maxcnt}").unwrap();
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
