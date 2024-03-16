struct Pair {
    ch: u8,
    count: u8,
}

fn permute(v: &mut [Pair], s: &mut String, n: usize) {
    if n == 0 {
        println!("{s}");
        return;
    }

    for i in 0..v.len() {
        if v[i].count != 0 {
            s.push(v[i].ch as char);
            v[i].count -= 1;
            permute(v, s, n - 1);
            s.pop();
            v[i].count += 1;
        }
    }
}

fn main() {
    let mut token = Tokenizer::new();
    let s: String = token.next();
    let mut v: Vec<Pair> = Vec::new();
    let n: usize = s.len();
    let mut c: u128 = 1;

    // this step eliminates sorting
    let mut h: Vec<u8> = vec![0; 128];
    for (i, ch) in s.chars().enumerate() {
        c *= i as u128 + 1;
        h[ch as usize] += 1;
    }

    // creating a shorter vector for shorter loop time
    for i in 0..128 {
        if h[i] != 0 {
            v.push(Pair {
                ch: i as u8,
                count: h[i] as u8,
            });
            for i in 2..=h[i] {
                c /= i as u128;
            }
        }
    }
    std::mem::drop(h);

    println!("{c}");
    permute(&mut v, &mut "".to_string(), n);
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
