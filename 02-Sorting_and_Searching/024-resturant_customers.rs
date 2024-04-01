use std::collections::BTreeMap;
use std::io::Write;

#[allow(unused_variables)]
fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n = token.next::<usize>();
    let mut map = BTreeMap::<i32, i32>::new();
    for _ in 0..n {
        map.entry(token.next::<i32>())
            .and_modify(|k| *k += 1)
            .or_insert(1);
        map.entry(token.next::<i32>())
            .and_modify(|k| *k -= 1)
            .or_insert(-1);
    }
    let mut mc = 0i32;
    let mut cc = 0i32;
    for (i, count) in &mut map {
        cc += *count;
        *count = cc;
        if mc < *count {
            mc = *count;
        }
    }
    writeln!(out, "{mc}").unwrap();
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
