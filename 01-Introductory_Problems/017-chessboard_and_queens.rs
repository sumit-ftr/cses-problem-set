use std::io::Write;

fn nqueen_variant(
    v: &[Vec<u8>],
    row: usize,
    sum: &mut usize,
    c: &mut [u8],
    ld: &mut [u8],
    rd: &mut [u8],
) {
    if row == v.len() {
        *sum += 1;
        return;
    }

    for col in 0..v.len() {
        if c[col] == 0 && rd[row + col] == 0 && ld[v.len() - 1 + col - row] == 0 && v[row][col] == 0
        {
            c[col] = 1;
            rd[row + col] = 1;
            ld[v.len() - 1 + col - row] = 1;
            nqueen_variant(v, row + 1, sum, c, ld, rd);
            c[col] = 0;
            rd[row + col] = 0;
            ld[v.len() - 1 + col - row] = 0;
        }
    }
    return;
}

#[allow(non_upper_case_globals)]
fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    const n: usize = 8;
    let mut v = vec![Vec::<u8>::with_capacity(n); n];
    let mut c = vec![0u8; n];
    let mut ld = vec![0u8; 2 * n - 1];
    let mut rd = vec![0u8; 2 * n - 1];
    let mut sum = 0usize;
    // 0 for free , 1 for queens , 2 for reserved
    for i in 0..n {
        let s = token.next::<String>();
        for ch in s.chars() {
            if ch == '.' {
                v[i].push(0);
            } else {
                v[i].push(2);
            }
        }
    }

    nqueen_variant(&v, 0, &mut sum, &mut c, &mut ld, &mut rd);
    writeln!(out, "{sum}").unwrap();
}

// fn is_safe(v: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
//     let m = usize::min(i, j);
//     for k in 0..v.len() {
//         // horizontal and vertical check
//         if v[k][j] == 2 || v[i][k] == 2 {
//             return false;
//         }
//         // left diagonal (\) check
//         if i-m+k < v.len() && j-m+k < v.len() {
//             if v[i-m+k][j-m+k] == 2 {
//                 return false;
//             }
//         }
//         // right diagonal (/) check
//         if i > k && j+k < v.len() {
//             if v[i-k][j+k] == 2 {
//                 return false;
//             }
//         }
//     }
//     return true;
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
