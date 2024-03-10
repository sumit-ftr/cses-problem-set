use std::collections::HashMap;

fn main() {
    let mut token = Tokenizer::new();
    let n = token.next::<usize>();
    let x = token.next::<usize>();
    let mut h = HashMap::<u32, u32>::with_capacity(n);
    let mut found = false;
    for index in 1..=n {
        let value = token.next::<u32>();
        if value == x as u32 - value {
            if let Some(&v) = h.get(&value) {
                println!("{} {}", v, index);
                found = true;
                break;
            }
        } else if let Some(&v) = h.get(&(x as u32 - value)) {
            println!("{} {}", v, index);
            found = true;
            break;
        }

        if value <= x as u32 {
            h.insert(value, index as u32);
        }
    }
    if !found {
        println!("IMPOSSIBLE");
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
