use std::io::Write;

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n = token.next::<usize>();
    let m = usize::pow(2, n as u32);
    let mut s = String::with_capacity(n);
    for _ in 0..n {
        s.push('0');
    }

    writeln!(out, "{s}").unwrap();
    for i in 1..m {
        let mut c = 0usize;
        while i & (1 << c) == 0 {
            c += 1;
        }

        if s.chars().nth(c).unwrap() == '0' {
            s.replace_range(c..c + 1, "1");
        } else {
            s.replace_range(c..c + 1, "0");
        }
        writeln!(out, "{s}").unwrap();
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
