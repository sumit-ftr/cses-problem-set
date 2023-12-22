fn main() {
    let mut token = Tokenizer::new();
    let n: usize = token.next();
    let l: usize = f64::ceil(n as f64/128.0) as usize;
    let mut v: Vec<u128> = Vec::with_capacity(l);

    for _ in 0..l { v.push(0); }
    v[0] |= 1;

    for _ in 1..n {
        let t: usize = token.next();
        let (i, j): (usize, usize) = (t/128, t%128);
        v[i] |= 1u128<<j;
    }

    for i in 0..l {
        if v[i] != std::u128::MAX {
            for j in 0..128 {
                if v[i]&(1<<j) == 0 {
                    println!("{}", j + i*128);
                    break;
                }
            }
            break;
        }
    }
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

