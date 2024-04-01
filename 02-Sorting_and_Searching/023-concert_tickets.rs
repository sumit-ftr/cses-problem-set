use std::collections::BTreeMap;
use std::io::Write;

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n = token.next::<usize>();
    let m = token.next::<usize>();
    let mut tickets = BTreeMap::<u32, u32>::new();
    for _ in 0..n {
        tickets
            .entry(token.next::<u32>())
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
    for _ in 0..m {
        let price = token.next::<u32>();
        if let Some((&k, v)) = tickets.range_mut(..=price).next_back() {
            writeln!(out, "{k}").unwrap();
            if *v == 1 {
                tickets.remove(&k);
            } else {
                *v -= 1;
            }
        } else {
            writeln!(out, "-1").unwrap();
        }
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
