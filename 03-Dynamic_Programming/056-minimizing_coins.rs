use std::io::Write;

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n = token.next::<usize>();
    let x = token.next::<usize>();
    let mut coins = Vec::<u32>::with_capacity(n);
    let mut dp = vec![1000001; x + 1];
    for _ in 0..n {
        let c = token.next::<u32>();
        if c as usize <= x {
            coins.push(c);
            dp[c as usize] = 1;
        }
    }
    coins.sort_unstable();

    dp[0] = 0;
    for i in 1..=x {
        let mut j = 0usize;
        while j < coins.len() && i >= coins[j] as usize {
            dp[i] = u32::min(dp[i], dp[i - coins[j] as usize] + 1);
            j += 1;
        }
    }
    writeln!(out, "{}", if dp[x] == 1000001 { -1 } else { dp[x] as i64 }).unwrap();
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
