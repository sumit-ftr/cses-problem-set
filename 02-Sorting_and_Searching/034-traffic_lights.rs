use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;

#[allow(dropping_references)]
fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let x = token.next::<u32>();
    let n = token.next::<usize>();
    let mut pos = BTreeSet::<u32>::from([0, x]);
    let mut max = BTreeMap::<u32, u32>::from([(x, 1)]);
    for _ in 0..n {
        let p = token.next::<u32>();
        pos.insert(p);
        let before = pos.range(..p).next_back().unwrap().clone();
        let after = pos.range(p + 1..).next().unwrap().clone();
        if let Some((k, v)) = max.range_mut(after - before..).next() {
            if *v == 1 {
                let key = k.clone();
                drop(k);
                drop(v);
                max.remove(&key);
            } else {
                *v -= 1;
            }
            max.entry(after - p).and_modify(|x| *x += 1).or_insert(1);
            max.entry(p - before).and_modify(|x| *x += 1).or_insert(1);
        }
        write!(out, "{} ", max.last_entry().unwrap().key()).unwrap();
    }
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
