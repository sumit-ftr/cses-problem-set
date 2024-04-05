use std::io::Write;

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n = token.next::<usize>();
    let k = token.next::<usize>();
    let sq = f64::sqrt(n as f64) as usize;
    let mut v = Vec::<Vec<u32>>::with_capacity((n + sq - 1) / sq);

    let mut t = Vec::<u32>::with_capacity(sq);
    for i in 1..=n {
        t.push(i as u32);
        if t.len() >= sq {
            v.push(std::mem::replace(&mut t, Vec::<u32>::with_capacity(sq)));
        }
    }
    if !t.is_empty() {
        v.push(t);
    }

    let (mut x, mut y) = (0usize, 0usize);
    for i in 0..n {
        y += k % (n - i);
        while y >= v[x].len() {
            y -= v[x].len();
            x += 1;
            if x == v.len() {
                x = 0;
            }
        }
        write!(out, "{} ", v[x][y]).unwrap();
        v[x].remove(y);
        if y == v.len() {
            y = 0;
            x += 1;
            if x == v.len() {
                x = 0;
            }
        }
    }
    writeln!(out, "").unwrap();
}

// use std::collections::BTreeSet;
// let mut set = BTreeSet::<u32>::new();
// for i in 1..=n {
//     set.insert(i as u32);
// }
// let mut i = 0;
// while set.len() > 0 {
//     i = (i + k) % set.len();
//     let item = *set.iter().nth(i).unwrap();
//     set.remove(&item);
//     write!(out, "{} ", item).unwrap();
// }

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
