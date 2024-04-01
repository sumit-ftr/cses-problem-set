use std::io::Write;

fn update(v: &[u32], mut fi: usize, mut se: usize) -> isize {
    let mut res = 0isize;
    if isize::abs(fi as isize - se as isize) >= 2 {
        if v[fi] > v[fi + 1] {
            res += 1;
        }
        if v[se - 1] > v[se] {
            res += 1;
        }
    } else {
        if fi > se {
            std::mem::swap(&mut fi, &mut se);
        }
        if v[fi] > v[se] {
            res += 1;
        }
    }
    if v[fi - 1] > v[fi] {
        res += 1;
    }
    if v[se] > v[se + 1] {
        res += 1;
    }
    res
}

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n = token.next::<usize>();
    let m = token.next::<usize>();
    let mut count = 1isize;
    let mut u = Vec::with_capacity(n + 2);
    let mut v = vec![0u32; n + 2];
    u.push(0);
    for i in 1..=n {
        u.push(token.next::<u32>());
        v[u[i] as usize] = i as u32;
    }
    u.push(n as u32 + 1);
    v[n + 1] = n as u32 + 1;
    for i in 1..n {
        if v[i] > v[i + 1] {
            count += 1;
        }
    }
    for _ in 0..m {
        let mut fi = token.next::<usize>();
        let mut se = token.next::<usize>();
        // swapping main value array
        u.swap(fi, se);

        (fi, se) = (u[se] as usize, u[fi] as usize);
        count -= update(&v, fi, se);

        // swapping mapped value array
        v.swap(fi, se);
        count += update(&v, fi, se);

        writeln!(out, "{}", count).unwrap();
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
