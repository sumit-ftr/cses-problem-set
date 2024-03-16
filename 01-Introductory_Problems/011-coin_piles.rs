fn main() {
    let mut token = Tokenizer::new();
    let t: u32 = token.next();
    for _ in 0..t {
        let mut a: u32 = token.next();
        let mut b: u32 = token.next();
        if a > b && a <= b << 1 {
            // remove one coin from right pile and two from left pile
            let d = a - b;
            a -= d + d;
            b -= d;
        } else if a < b && a << 1 >= b {
            // remove one coin from left pile and two from right pile
            let d = b - a;
            a -= d;
            b -= d + d;
        }

        if a == b && a % 3 == 0 {
            println!("YES");
        } else {
            println!("NO");
        }
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
