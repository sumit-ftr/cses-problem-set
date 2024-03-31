use std::io::Write;

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n = token.next::<u64>();

    if n >= 1 {
        writeln!(out, "0").unwrap();
    }

    if n >= 2 {
        // (4 * (n^2 - 1 - 0)) / 2
        writeln!(out, "6").unwrap();
    }

    if n >= 3 {
        // (8 * (n^2 - 1 - 2) + 1 * (n^2 - 1 - 0)) / 2
        writeln!(out, "28").unwrap();
    }

    for i in 4..=n {
        let mut sum: u64 = 0;

        // 2x2 corners
        // 4 * ((n^2 - 1 - 2) + (n^2 - 1 - 3) + (n^2 - 1 - 3) + (n^2 - 1 - 4))
        sum += 16 * (i * i - 4);

        // 2x(n-4) edges
        // 4 * (n - 4) * ((n^2 - 1 - 4) + (n^2 - 1 - 6))
        sum += 8 * (i - 4) * (i * i - 6);

        // (n-4)x(n-4) centers
        // (n - 4) * (n - 4) * (n^2 - 1 - 8)
        sum += (i - 4) * (i - 4) * (i * i - 9);

        sum /= 2;
        writeln!(out, "{sum}").unwrap();
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
