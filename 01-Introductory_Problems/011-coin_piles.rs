use std::io::Write;

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let t = token.next::<u32>();
    for _ in 0..t {
        let mut a = token.next::<u32>();
        let mut b = token.next::<u32>();
        if a > b && a <= b << 1 {
            // remove one coin from right pile and two from left pile
            let d = a - b;
            a -= d + d;
            b -= d;
        } else if a < b && a << 1 >= b {
            // remove one coin from left pile and two from right pile
            let d = b - a;
            a -= d;
            b -= d + d;
        }

        if a == b && a % 3 == 0 {
            writeln!(out, "YES").unwrap();
        } else {
            writeln!(out, "NO").unwrap();
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
