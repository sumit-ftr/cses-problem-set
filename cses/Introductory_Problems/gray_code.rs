fn main() {
    let mut token = Tokenizer::new();
    let n: usize = token.next();
    let m: usize = usize::pow(2, n as u32);
    let mut s: String = String::with_capacity(n);
    for _ in 0..n {
        s.push('0');
    }

    println!("{s}");
    for i in 1..m {
        let mut c: usize = 0;
        while i&(1<<c) == 0 {
            c += 1;
        }

        if s.chars().nth(c).unwrap() == '0' {
            s.replace_range(c..c+1, "1");
        } else {
            s.replace_range(c..c+1, "0");
        }
        println!("{s}");
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

