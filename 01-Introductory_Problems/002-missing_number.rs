use std::io::Write;

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n = token.next::<u128>();
    let mut sum: u128 = n * (n + 1) / 2;
    for _ in 1..n {
        let val = token.next::<u32>();
        sum -= val as u128;
    }
    writeln!(out, "{sum}").unwrap();
}

// fn main() {
//     let mut token = Scanner::new(std::io::stdin().lock());
//     let n: usize = token.next();
//     let l: usize = f64::ceil(n as f64 / 128.0) as usize;
//     let mut v: Vec<u128> = Vec::with_capacity(l);
//     for _ in 0..l {
//         v.push(0);
//     }
//     v[0] |= 1;
//     for _ in 1..n {
//         let t: usize = token.next();
//         let (i, j): (usize, usize) = (t / 128, t % 128);
//         v[i] |= 1u128 << j;
//     }
//     for i in 0..l {
//         if v[i] != std::u128::MAX {
//             for j in 0..128 {
//                 if v[i] & (1 << j) == 0 {
//                     println!("{}", j + i * 128);
//                     break;
//                 }
//             }
//             break;
//         }
//     }
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
