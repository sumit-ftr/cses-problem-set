use std::io::Write;

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
}

pub struct Scanner<'a> {
    #[allow(dead_code)]
    buffer: Vec<u8>,
    iter: std::str::SplitAsciiWhitespace<'a>,
}

impl Scanner<'_> {
    pub fn new<R: std::io::Read>(mut reader: R) -> Self {
        let mut buffer = vec![];
        unsafe {
            reader.read_to_end(&mut buffer).unwrap_unchecked();
        }
        let iter = unsafe {
            let slice = std::str::from_utf8_unchecked(&buffer);
            std::mem::transmute(slice.split_ascii_whitespace())
        };

        Self { buffer, iter }
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        unsafe {
            self.iter
                .next()
                .unwrap_unchecked()
                .parse()
                .unwrap_unchecked()
        }
    }
}
