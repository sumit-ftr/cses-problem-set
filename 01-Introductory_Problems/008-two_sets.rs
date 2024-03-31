use std::io::Write;

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n = token.next::<usize>();
    if n & 3 == 1 || n & 3 == 2 {
        writeln!(out, "NO").unwrap();
    } else {
        writeln!(out, "YES").unwrap();
        if n & 3 == 0 {
            writeln!(out, "{}", n / 2).unwrap();
            for i in (1..=n).step_by(4) {
                write!(out, "{} {} ", i, i + 3).unwrap();
            }
            writeln!(out, "\n{}", n / 2).unwrap();
            for i in (1..=n).step_by(4) {
                write!(out, "{} {} ", i + 1, i + 2).unwrap();
            }
            writeln!(out, "").unwrap();
        } else {
            writeln!(out, "{}", (n + 1) / 2).unwrap();
            write!(out, "1 2 ").unwrap();
            for i in (4..=n).step_by(4) {
                write!(out, "{} {} ", i, i + 3).unwrap();
            }
            writeln!(out, "\n{}", n / 2).unwrap();
            write!(out, "3 ").unwrap();
            for i in (4..=n).step_by(4) {
                write!(out, "{} {} ", i + 1, i + 2).unwrap();
            }
            writeln!(out, "").unwrap();
        }
    }
}

/*
fn main() {
    let mut token = Tokenizer::new();
    let n: usize = token.next();
    let sum: u128 = (n*(n+1)/2) as u128;
    if sum&1 == 0 {
        println!("YES");
        let mut sum2: u128 = 0;
        let mut v1: Vec<u32> = Vec::with_capacity(n/4);
        let mut v2: Vec<u32> = Vec::with_capacity(n/2);
        for i in (1..=n as u32).rev() {
            if sum2 + i as u128 <= sum/2 {
                sum2 += i as u128;
                v1.push(i);
            } else {
                v2.push(i);
            }
        }

        println!("{}", v1.len());
        for i in 0..v1.len() {
            print!("{} ", v1[i]);
        }
        println!("");
        println!("{}", v2.len());
        for i in 0..v2.len() {
            print!("{} ", v2[i]);
        }
        println!("");
    } else {
        println!("NO");
    }
}
*/

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
