use std::io::Write;

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let mut v = Vec::<usize>::with_capacity(19);
    v.push(0);
    for i in 1..=18 {
        v.push(v[i - 1] + i * 9 * usize::pow(10, i as u32 - 1));
    }

    let t = token.next::<u16>();
    for _ in 0..t {
        let k = token.next::<usize>();
        let mut i = 0usize;
        while v[i] < k {
            i += 1;
        }

        // 10^i-1 <= k < 10^i
        let mut num: usize = (k - v[i - 1] + (i - 1)) / i;
        if i != 1 {
            num += usize::pow(10, i as u32 - 1) - 1;
        }

        let div: usize = (k - v[i - 1]) % i;
        if div == 0 {
            writeln!(out, "{}", num % 10).unwrap();
        } else {
            let res: usize = (num / usize::pow(10, (i - div) as u32)) % 10;
            writeln!(out, "{res}").unwrap();
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
