fn main() {
    let mut token = Tokenizer::new();
    let n: u64 = token.next();

    if n >= 1 {
        println!("0");
    }

    if n >= 2 {
        // (4 * (n^2 - 1 - 0)) / 2
        println!("6");
    }

    if n >= 3 {
        // (8 * (n^2 - 1 - 2) + 1 * (n^2 - 1 - 0)) / 2
        println!("28");
    }

    for i in 4..=n {
        let mut sum: u64 = 0;

        // 2x2 corners
        // 4 * ((n^2 - 1 - 2) + (n^2 - 1 - 3) + (n^2 - 1 - 3) + (n^2 - 1 - 4))
        sum += 16 * (i * i - 4);

        // 2x(n-4) edges
        // 4 * (n - 4) * ((n^2 - 1 - 4) + (n^2 - 1 - 6))
        sum += 8 * (i - 4) * (i * i - 6);

        // (n-4)x(n-4) centers
        // (n - 4) * (n - 4) * (n^2 - 1 - 8)
        sum += (i - 4) * (i - 4) * (i * i - 9);

        sum /= 2;
        println!("{}", sum);
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
