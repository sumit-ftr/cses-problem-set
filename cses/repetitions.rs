fn main() {
    let mut token = Tokenizer::new();
    let v: Vec<u8> = token.next::<String>().into();
    let mut max_count = 1;
    let mut cur_count = 1;
    let mut ch = v[0];
    for i in 1..v.len() {
        if ch == v[i] {
            cur_count += 1;
            if cur_count > max_count {
                max_count = cur_count;
            }
        } else {
            cur_count = 1;
            ch = v[i];
        }
    }
    println!("{}", max_count);
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

