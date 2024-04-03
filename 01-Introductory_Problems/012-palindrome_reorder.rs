use std::io::Write;

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let s = token.next::<String>();
    // hash taken for storing count of A-Z
    let mut a: [u32; 26] = [0; 26];
    for c in s.chars() {
        a[c as usize - 65] += 1;
    }

    // pre-computation for checking palindrome possiblility
    let mut odd_count = 0u32;
    for i in 0..26 {
        odd_count += a[i] & 1;
    }

    // checking reordering palindrome possiblity
    if odd_count > 1 {
        writeln!(out, "NO SOLUTION").unwrap();
    } else {
        let mut odd_index: usize = 26;
        for i in 0..26 {
            if a[i] & 1 == 0 {
                a[i] >>= 1;
                for _ in 0..a[i] {
                    write!(out, "{}", char::from_u32(i as u32 + 65).unwrap()).unwrap();
                }
            } else {
                odd_index = i;
            }
        }
        if odd_index != 26 {
            for _ in 0..a[odd_index] {
                write!(out, "{}", char::from_u32(odd_index as u32 + 65).unwrap()).unwrap();
            }
            a[odd_index] = 0;
        }
        for i in (0..26).rev() {
            for _ in 0..a[i] {
                write!(out, "{}", char::from_u32(i as u32 + 65).unwrap()).unwrap();
            }
        }
        writeln!(out, "").unwrap();
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
