struct Pair {
    s: u32,
    e: u32
}

fn main() {
    let mut token = Tokenizer::new();
    let n: usize = token.next();
    let mut v: Vec<Pair> = Vec::with_capacity(n);
    for _ in 0..n {
        let p: Pair = Pair {
            s: token.next(),
            e: token.next()
        };
        v.push(p);
    }
    v.sort_by(|a, b| (a.e).cmp(&b.e).then((a.s).cmp(&b.s)));
    let mut lmet: u32 = 0; // last movie ending time
    let mut count: u32 = 0;
    for i in 0..n {
        if lmet <= v[i].s {
            lmet = v[i].e;
            count += 1;
        }
    }
    println!("{count}");
}

struct Tokenizer {
    buf: Vec<String>,
    i: usize
}

impl Tokenizer {
    pub fn new() -> Self {
        return Tokenizer { buf: Vec::<String>::new(), i: 0 };
    }

    fn read_line(&mut self) {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        self.buf = s.split_whitespace().map(str::to_string).collect();
        self.i = 0;
    }

    pub fn next<T : std::str::FromStr>(&mut self) -> T
    where T::Err : std::fmt::Debug {
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

