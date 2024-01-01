fn main() {
    let mut token = Tokenizer::new();
    let mut v: Vec<usize> = Vec::with_capacity(19);
    v.push(0);
    for i in 1..=18 {
        v.push(v[i-1] + i*9 * usize::pow(10, i as u32-1));
    }

    let t: u16 = token.next();
    for _ in 0..t {
        let k: usize = token.next();
        let mut i: usize = 0;
        while v[i] < k {
            i += 1;
        }

        // 10^i-1 <= k < 10^i
        let mut num: usize = (k - v[i-1] + (i-1)) / i;
        if i != 1 {
            num += usize::pow(10, i as u32-1) - 1;
        }

        let div: usize = (k - v[i-1]) % i;
        if div == 0 {
            println!("{}", num%10);
        } else {
            let res: usize = (num / usize::pow(10, (i-div) as u32)) % 10;
            println!("{}", res);
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

