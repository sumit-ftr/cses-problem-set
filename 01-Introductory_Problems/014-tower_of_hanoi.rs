fn tower_of_hanoi(n: u8, start: u8, end: u8) {
    if n == 1 {
        println!("{start} {end}");
        return;
    }

    let other: u8 = 6 - (start+end);
    tower_of_hanoi(n-1, start, other);
    println!("{start} {end}");
    tower_of_hanoi(n-1, other, end);
}

fn main() {
    let mut token = Tokenizer::new();
    let n: u8 = token.next();
    let moves: u32 = u32::pow(2, n as u32) - 1;
    println!("{moves}");
    tower_of_hanoi(n, 1, 3);
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

