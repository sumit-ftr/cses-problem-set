fn divide(v: &[u32], i: usize, s1: u64, s2: u64, mid: u64) -> u64 {
    if i == v.len() {
        if s1 > s2 {
            return s1 - s2;
        } else {
            return s2 - s1;
        }
    }

    let mut d1: u64 = u64::MAX;
    let mut d2: u64 = u64::MAX;
    if s1 < mid {
        d1 = u64::min(d1, divide(v, i + 1, s1 + v[i] as u64, s2, mid));
    }
    if s2 < mid {
        d2 = u64::min(d2, divide(v, i + 1, s1, s2 + v[i] as u64, mid));
    }
    return u64::min(d1, d2);
}

fn main() {
    let mut token = Tokenizer::new();
    let n: usize = token.next();
    let mut v: Vec<u32> = Vec::with_capacity(n);
    let mut sum: u64 = 0;
    for i in 0..n {
        v.push(token.next());
        sum += v[i] as u64;
    }

    if n == 1 {
        println!("{}", v[0]);
    } else {
        // let diff: u64 = divide(&v, 0, 0, 0, sum.div_ceil(2));
        let diff: u64 = divide(&v, 0, 0, 0, (sum + 1) / 2);
        println!("{diff}");
    }
}

struct Tokenizer {
    buf: Vec<String>,
    i: usize,
}

impl Tokenizer {
    pub fn new() -> Self {
        return Tokenizer {
            buf: Vec::<String>::new(),
            i: 0,
        };
    }

    fn read_line(&mut self) {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        self.buf = s.split_whitespace().map(str::to_string).collect();
        self.i = 0;
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        while self.i == self.buf.len() {
            self.read_line();
        }
        let t = self.buf[self.i].parse().unwrap();
        self.i += 1;
        return t;
    }

    #[allow(dead_code)]
    pub fn next_line(&self) -> String {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        return s;
    }
}
