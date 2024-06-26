use std::collections::HashMap;
use std::io::Write;

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n = token.next::<usize>();
    let x = token.next::<usize>();
    let mut h = HashMap::<u32, u32>::with_capacity(n);
    let mut found = false;
    for index in 1..=n {
        let value = token.next::<u32>();
        if value == x as u32 - value {
            if let Some(&v) = h.get(&value) {
                writeln!(out, "{v} {index}").unwrap();
                found = true;
                break;
            }
        } else if let Some(&v) = h.get(&(x as u32 - value)) {
            writeln!(out, "{v} {index}").unwrap();
            found = true;
            break;
        }

        if value <= x as u32 {
            h.insert(value, index as u32);
        }
    }
    if !found {
        writeln!(out, "IMPOSSIBLE").unwrap();
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
