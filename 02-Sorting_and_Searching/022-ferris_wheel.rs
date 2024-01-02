fn main() {
    let mut token = Tokenizer::new();
    let n: usize = token.next();
    let x: usize = token.next();
    let mut v: Vec<u32> = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(token.next());
    }
    v.sort();

    let mut l: usize = 0;
    let mut r: usize = n-1;
    let mut count: usize = 0;
    while l<=r {
        if v[l] as usize+v[r] as usize <= x {
            l += 1;
        }
        count += 1;
        if r==0 {
            break;
        }
        r -= 1;
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

