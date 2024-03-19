fn diff_abs(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn check(v: &[u32], mut fi: usize, mut se: usize, n: usize) -> usize {
    let mut res = 0usize;
    if diff_abs(fi, se) >= 2 {
        if fi < n && v[fi] > v[fi + 1] {
            res += 1;
        }
        if se > 1 && v[se - 1] > v[se] {
            res += 1;
        }
    } else {
        if fi > se {
            (fi, se) = (se, fi);
        }
        if v[fi] > v[se] {
            res += 1;
        }
    }
    if fi > 1 && v[fi - 1] > v[fi] {
        res += 1;
    }
    if se < n && v[se] > v[se + 1] {
        res += 1;
    }
    res
}

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let n = token.next::<usize>();
    let m = token.next::<usize>();
    let mut u = Vec::with_capacity(n + 1);
    let mut v = vec![0u32; n + 1];
    u.push(0);
    for i in 1..=n {
        let value = token.next::<usize>();
        u.push(value as u32);
        v[value] = i as u32;
    }
    let mut count = 1usize;
    for i in 1..n {
        if v[i] > v[i + 1] {
            count += 1;
        }
    }
    for _ in 0..m {
        let mut fi = token.next::<usize>();
        let mut se = token.next::<usize>();
        // swapping main value array
        (u[fi], u[se]) = (u[se], u[fi]);

        (fi, se) = (u[se] as usize, u[fi] as usize);
        let dec = check(&v, fi, se, n);

        // swapping mapped value array
        (v[fi], v[se]) = (v[se], v[fi]);
        let inc = check(&v, fi, se, n);

        count = (count as isize + (inc as isize - dec as isize)) as usize;
        println!("{}", count);
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
