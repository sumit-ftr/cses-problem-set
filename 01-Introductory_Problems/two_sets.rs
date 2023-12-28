fn main() {
    let mut token = Tokenizer::new();
    let n: u32 = token.next();
    let sum: u128 = ((n as f64/2.0) * (n as f64 + 1.0)) as u128;
    if sum&1 == 0 {
        println!("YES");
        let mut sum2: u128 = 0;
        let mut v1: Vec<u32> = Vec::with_capacity((n/4) as usize);
        let mut v2: Vec<u32> = Vec::with_capacity((n/2) as usize);
        for i in (1..=n).rev() {
            if sum2 + i as u128 <= sum/2 {
                sum2 += i as u128;
                v1.push(i);
            } else {
                v2.push(i);
            }
        }

        println!("{}", v1.len());
        for i in 0..v1.len() {
            print!("{} ", v1[i]);
        }
        println!("");
        println!("{}", v2.len());
        for i in 0..v2.len() {
            print!("{} ", v2[i]);
        }
        println!("");
    } else {
        println!("NO");
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

