fn main() {
    let mut token = Tokenizer::new();
    let n: usize = token.next();
    let m: usize = token.next();
    let k: u32 = token.next();
    let mut applicants: Vec<u32> = Vec::with_capacity(n);
    let mut apartments: Vec<u32> = Vec::with_capacity(m);
    for _ in 0..n {
        applicants.push(token.next());
    }
    for _ in 0..m {
        apartments.push(token.next());
    }
    applicants.sort();
    apartments.sort();

    let mut reserved: u32 = 0;
    let mut i: usize = 0;
    let mut j: usize = 0;
    while i < n && j < m {
        if applicants[i] as i64 - k as i64 <= apartments[j] as i64
            && apartments[j] <= applicants[i] + k
        {
            i += 1;
            j += 1;
            reserved += 1;
        } else if applicants[i] as i64 - k as i64 > apartments[j] as i64 {
            j += 1;
        } else {
            i += 1;
        }
    }
    println!("{reserved}");
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
