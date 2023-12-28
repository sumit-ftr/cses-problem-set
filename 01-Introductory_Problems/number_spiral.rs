fn main() {
    let mut token = Tokenizer::new();
    let t: u32 = token.next();
    for _ in 0..t {
        let x: u32 = token.next();
        let y: u32 = token.next();
        let result: u128;
        let max: u128 = if x > y { x as u128 } else { y as u128 };
        // (max-1)^2 .. max^2
        if max&1 == 0 {
            // starts from upside
            if x > y {
                result = u128::pow(max, 2) - (y-1) as u128;
            } else {
                result = u128::pow(max-1, 2) + x as u128;
            }
        } else {
            // starts from leftside
            if x > y {
                result = u128::pow(max-1, 2) + y as u128;
            } else {
                result = u128::pow(max, 2) - (x-1) as u128;
            }
        }
        println!("{}", result);
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

