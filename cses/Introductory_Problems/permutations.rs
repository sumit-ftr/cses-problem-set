fn main() {
    let mut token = Tokenizer::new();
    let n: usize = token.next();
    if n == 2 || n == 3 {
        println!("NO SOLUTION");
    } else if n == 1 {
        println!("1");
    } else {
        let mut v: Vec<u32> = Vec::with_capacity(n);
        v.push(2);
        for i in 1..n/2 {
            v.push(v[i-1] + 2);
        }
        for i in 0..n/2 {
            v.push((i*2+1) as u32);
        }
        if n&1 == 1 {
            v.push(n as u32);
        }

        println!();
        for i in 0..n {
            print!("{} ", v[i]);
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

