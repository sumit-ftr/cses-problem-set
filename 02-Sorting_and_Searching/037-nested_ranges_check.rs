use std::io::Write;

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n = token.next::<usize>();
    let mut v = Vec::<(u32, u32)>::with_capacity(n);
    // contains other range
    let mut inr = Vec::<u8>::with_capacity(n);
    // other range contains it
    let mut rin = Vec::<u8>::with_capacity(n);
    let mut h = BTreeMap::<u32, BTreeSet<u32>>::new();
    for _ in 0..n {
        let x = token.next::<u32>();
        let y = token.next::<u32>();
        v.push((x, y));
        h.entry(x)
            .and_modify(|set| set.insert(y))
            .or_insert(BTreeSet::from([y]));
    }

    for index in 0..n {
        let gei = h.range(v[index].0..);
        if h.next().unwrap().1.range(..=v[index].1).count() > 0 {
            inr.push(1);
        }
        // let gi_lej = ;
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
